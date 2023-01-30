[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 18,
                            end: 24,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 26,
                            end: 32,
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
            src (ptr): 0x00007fb0e931aa40,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
            ),
            start: 9,
            end: 33,
            as_str(): "use std::assert::assert;",
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
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 38,
                            end: 42,
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 57,
                                                                end: 63,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 57,
                                                        end: 63,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 81,
                                                                                        end: 83,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 81,
                                                                                        end: 83,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 83,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 81,
                                                                        end: 83,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 64,
                                                                                        end: 68,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 64,
                                                                                        end: 80,
                                                                                        as_str(): "__eq(true, true)",
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 69,
                                                                                            end: 73,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 75,
                                                                                            end: 79,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 64,
                                                                            end: 80,
                                                                            as_str(): "__eq(true, true)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 90,
                                                                                                        end: 92,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 90,
                                                                                                        end: 92,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 90,
                                                                                                    end: 92,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 90,
                                                                                        end: 92,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                true,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 85,
                                                                                            end: 89,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 93,
                                                                                            end: 97,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 85,
                                                                            end: 97,
                                                                            as_str(): "true == true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 64,
                                                            end: 98,
                                                            as_str(): "__eq(true, true) == (true == true)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 57,
                                            end: 99,
                                            as_str(): "assert(__eq(true, true) == (true == true))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 57,
                                    end: 99,
                                    as_str(): "assert(__eq(true, true) == (true == true))",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 103,
                                                                end: 109,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 103,
                                                        end: 109,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 128,
                                                                                        end: 130,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 128,
                                                                                        end: 130,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "neq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 128,
                                                                                    end: 130,
                                                                                    as_str(): "!=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 128,
                                                                        end: 130,
                                                                        as_str(): "!=",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 110,
                                                                                        end: 114,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 110,
                                                                                        end: 127,
                                                                                        as_str(): "__eq(true, false)",
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 115,
                                                                                            end: 119,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 121,
                                                                                            end: 126,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 110,
                                                                            end: 127,
                                                                            as_str(): "__eq(true, false)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 137,
                                                                                                        end: 139,
                                                                                                        as_str(): "!=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 137,
                                                                                                        end: 139,
                                                                                                        as_str(): "!=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "neq",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 137,
                                                                                                    end: 139,
                                                                                                    as_str(): "!=",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 137,
                                                                                        end: 139,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                true,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 132,
                                                                                            end: 136,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 140,
                                                                                            end: 145,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 132,
                                                                            end: 145,
                                                                            as_str(): "true != false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 110,
                                                            end: 146,
                                                            as_str(): "__eq(true, false) != (true != false)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 103,
                                            end: 147,
                                            as_str(): "assert(__eq(true, false) != (true != false))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 103,
                                    end: 147,
                                    as_str(): "assert(__eq(true, false) != (true != false))",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 151,
                                                                end: 157,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 151,
                                                        end: 157,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 175,
                                                                                        end: 177,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 175,
                                                                                        end: 177,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 175,
                                                                                    end: 177,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 177,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 158,
                                                                                        end: 162,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 158,
                                                                                        end: 174,
                                                                                        as_str(): "__eq(true, true)",
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 163,
                                                                                            end: 167,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 169,
                                                                                            end: 173,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 158,
                                                                            end: 174,
                                                                            as_str(): "__eq(true, true)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 184,
                                                                                                        end: 186,
                                                                                                        as_str(): "!=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 184,
                                                                                                        end: 186,
                                                                                                        as_str(): "!=",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "neq",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 184,
                                                                                                    end: 186,
                                                                                                    as_str(): "!=",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 184,
                                                                                        end: 186,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                true,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 179,
                                                                                            end: 183,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 187,
                                                                                            end: 192,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 179,
                                                                            end: 192,
                                                                            as_str(): "true != false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 193,
                                                            as_str(): "__eq(true, true) == (true != false)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 151,
                                            end: 194,
                                            as_str(): "assert(__eq(true, true) == (true != false))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 151,
                                    end: 194,
                                    as_str(): "assert(__eq(true, true) == (true != false))",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 199,
                                                                end: 205,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 199,
                                                        end: 205,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 218,
                                                                                        end: 220,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 218,
                                                                                        end: 220,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 218,
                                                                                    end: 220,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 218,
                                                                        end: 220,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 206,
                                                                                        end: 210,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 206,
                                                                                        end: 217,
                                                                                        as_str(): "__eq(1, 22)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 211,
                                                                                            end: 212,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                22,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 214,
                                                                                            end: 216,
                                                                                            as_str(): "22",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 206,
                                                                            end: 217,
                                                                            as_str(): "__eq(1, 22)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 224,
                                                                                                        end: 226,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 224,
                                                                                                        end: 226,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 224,
                                                                                                    end: 226,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 224,
                                                                                        end: 226,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 222,
                                                                                            end: 223,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                22,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 227,
                                                                                            end: 229,
                                                                                            as_str(): "22",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 222,
                                                                            end: 229,
                                                                            as_str(): "1 == 22",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 230,
                                                            as_str(): "__eq(1, 22) == (1 == 22)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 199,
                                            end: 231,
                                            as_str(): "assert(__eq(1, 22) == (1 == 22))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 199,
                                    end: 231,
                                    as_str(): "assert(__eq(1, 22) == (1 == 22))",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 235,
                                                                end: 241,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 235,
                                                        end: 241,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 253,
                                                                                        end: 255,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 253,
                                                                                        end: 255,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 253,
                                                                                    end: 255,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 253,
                                                                        end: 255,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 242,
                                                                                        end: 246,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 242,
                                                                                        end: 252,
                                                                                        as_str(): "__eq(1, 1)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 247,
                                                                                            end: 248,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 250,
                                                                                            end: 251,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 242,
                                                                            end: 252,
                                                                            as_str(): "__eq(1, 1)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 259,
                                                                                                        end: 261,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 259,
                                                                                                        end: 261,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 259,
                                                                                                    end: 261,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 259,
                                                                                        end: 261,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 257,
                                                                                            end: 258,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 262,
                                                                                            end: 263,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 257,
                                                                            end: 263,
                                                                            as_str(): "1 == 1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 242,
                                                            end: 264,
                                                            as_str(): "__eq(1, 1) == (1 == 1)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 235,
                                            end: 265,
                                            as_str(): "assert(__eq(1, 1) == (1 == 1))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 235,
                                    end: 265,
                                    as_str(): "assert(__eq(1, 1) == (1 == 1))",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 274,
                                                    end: 275,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Eight,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 277,
                                                    end: 279,
                                                    as_str(): "u8",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 282,
                                                    end: 283,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 270,
                                    end: 284,
                                    as_str(): "let a: u8 = 1;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 291,
                                                    end: 292,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Eight,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 294,
                                                    end: 296,
                                                    as_str(): "u8",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        22,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 299,
                                                    end: 301,
                                                    as_str(): "22",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 287,
                                    end: 302,
                                    as_str(): "let b: u8 = 22;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 309,
                                                    end: 310,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Eight,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 312,
                                                    end: 314,
                                                    as_str(): "u8",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 317,
                                                    end: 318,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 305,
                                    end: 319,
                                    as_str(): "let c: u8 = 1;",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 322,
                                                                end: 328,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 322,
                                                        end: 328,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 340,
                                                                                        end: 342,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 340,
                                                                                        end: 342,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 340,
                                                                                    end: 342,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 340,
                                                                        end: 342,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 329,
                                                                                        end: 333,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 329,
                                                                                        end: 339,
                                                                                        as_str(): "__eq(a, b)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 334,
                                                                                                    end: 335,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 334,
                                                                                            end: 335,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 337,
                                                                                                    end: 338,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 337,
                                                                                            end: 338,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 329,
                                                                            end: 339,
                                                                            as_str(): "__eq(a, b)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 346,
                                                                                                        end: 348,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 346,
                                                                                                        end: 348,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 346,
                                                                                                    end: 348,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 346,
                                                                                        end: 348,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 344,
                                                                                                    end: 345,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 344,
                                                                                            end: 345,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 349,
                                                                                                    end: 350,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 349,
                                                                                            end: 350,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 344,
                                                                            end: 350,
                                                                            as_str(): "a == b",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 329,
                                                            end: 351,
                                                            as_str(): "__eq(a, b) == (a == b)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 322,
                                            end: 352,
                                            as_str(): "assert(__eq(a, b) == (a == b))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 322,
                                    end: 352,
                                    as_str(): "assert(__eq(a, b) == (a == b))",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 356,
                                                                end: 362,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 356,
                                                        end: 362,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 374,
                                                                                        end: 376,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 374,
                                                                                        end: 376,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 374,
                                                                                    end: 376,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 374,
                                                                        end: 376,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 363,
                                                                                        end: 367,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 363,
                                                                                        end: 373,
                                                                                        as_str(): "__eq(a, c)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 368,
                                                                                                    end: 369,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 368,
                                                                                            end: 369,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 371,
                                                                                                    end: 372,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 371,
                                                                                            end: 372,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 363,
                                                                            end: 373,
                                                                            as_str(): "__eq(a, c)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 380,
                                                                                                        end: 382,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 380,
                                                                                                        end: 382,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 380,
                                                                                                    end: 382,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 380,
                                                                                        end: 382,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 378,
                                                                                                    end: 379,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 378,
                                                                                            end: 379,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 383,
                                                                                                    end: 384,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 383,
                                                                                            end: 384,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 378,
                                                                            end: 384,
                                                                            as_str(): "a == c",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 363,
                                                            end: 385,
                                                            as_str(): "__eq(a, c) == (a == c)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 356,
                                            end: 386,
                                            as_str(): "assert(__eq(a, c) == (a == c))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 356,
                                    end: 386,
                                    as_str(): "assert(__eq(a, c) == (a == c))",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 395,
                                                    end: 396,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Sixteen,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 398,
                                                    end: 401,
                                                    as_str(): "u16",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 404,
                                                    end: 405,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 391,
                                    end: 406,
                                    as_str(): "let a: u16 = 1;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 413,
                                                    end: 414,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Sixteen,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 416,
                                                    end: 419,
                                                    as_str(): "u16",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        22,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 422,
                                                    end: 424,
                                                    as_str(): "22",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 409,
                                    end: 425,
                                    as_str(): "let b: u16 = 22;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 432,
                                                    end: 433,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Sixteen,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 435,
                                                    end: 438,
                                                    as_str(): "u16",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 441,
                                                    end: 442,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 428,
                                    end: 443,
                                    as_str(): "let c: u16 = 1;",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 446,
                                                                end: 452,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 446,
                                                        end: 452,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 464,
                                                                                        end: 466,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 464,
                                                                                        end: 466,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 464,
                                                                                    end: 466,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 464,
                                                                        end: 466,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 453,
                                                                                        end: 457,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 453,
                                                                                        end: 463,
                                                                                        as_str(): "__eq(a, b)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 458,
                                                                                                    end: 459,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 458,
                                                                                            end: 459,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 461,
                                                                                                    end: 462,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 461,
                                                                                            end: 462,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 453,
                                                                            end: 463,
                                                                            as_str(): "__eq(a, b)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 470,
                                                                                                        end: 472,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 470,
                                                                                                        end: 472,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 470,
                                                                                                    end: 472,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 470,
                                                                                        end: 472,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 468,
                                                                                                    end: 469,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 468,
                                                                                            end: 469,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 473,
                                                                                                    end: 474,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 473,
                                                                                            end: 474,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 468,
                                                                            end: 474,
                                                                            as_str(): "a == b",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 453,
                                                            end: 475,
                                                            as_str(): "__eq(a, b) == (a == b)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 446,
                                            end: 476,
                                            as_str(): "assert(__eq(a, b) == (a == b))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 446,
                                    end: 476,
                                    as_str(): "assert(__eq(a, b) == (a == b))",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 480,
                                                                end: 486,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 480,
                                                        end: 486,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 498,
                                                                                        end: 500,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 498,
                                                                                        end: 500,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 498,
                                                                                    end: 500,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 498,
                                                                        end: 500,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 487,
                                                                                        end: 491,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 487,
                                                                                        end: 497,
                                                                                        as_str(): "__eq(a, c)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 492,
                                                                                                    end: 493,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 492,
                                                                                            end: 493,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 495,
                                                                                                    end: 496,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 495,
                                                                                            end: 496,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 487,
                                                                            end: 497,
                                                                            as_str(): "__eq(a, c)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 504,
                                                                                                        end: 506,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 504,
                                                                                                        end: 506,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 504,
                                                                                                    end: 506,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 504,
                                                                                        end: 506,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 502,
                                                                                                    end: 503,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 502,
                                                                                            end: 503,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 507,
                                                                                                    end: 508,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 507,
                                                                                            end: 508,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 502,
                                                                            end: 508,
                                                                            as_str(): "a == c",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 487,
                                                            end: 509,
                                                            as_str(): "__eq(a, c) == (a == c)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 480,
                                            end: 510,
                                            as_str(): "assert(__eq(a, c) == (a == c))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 480,
                                    end: 510,
                                    as_str(): "assert(__eq(a, c) == (a == c))",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 519,
                                                    end: 520,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                ThirtyTwo,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 522,
                                                    end: 525,
                                                    as_str(): "u32",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 528,
                                                    end: 529,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 515,
                                    end: 530,
                                    as_str(): "let a: u32 = 1;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 537,
                                                    end: 538,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                ThirtyTwo,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 540,
                                                    end: 543,
                                                    as_str(): "u32",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        22,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 546,
                                                    end: 548,
                                                    as_str(): "22",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 533,
                                    end: 549,
                                    as_str(): "let b: u32 = 22;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 556,
                                                    end: 557,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                ThirtyTwo,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 559,
                                                    end: 562,
                                                    as_str(): "u32",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 565,
                                                    end: 566,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 552,
                                    end: 567,
                                    as_str(): "let c: u32 = 1;",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 570,
                                                                end: 576,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 570,
                                                        end: 576,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 588,
                                                                                        end: 590,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 588,
                                                                                        end: 590,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 588,
                                                                                    end: 590,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 588,
                                                                        end: 590,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 577,
                                                                                        end: 581,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 577,
                                                                                        end: 587,
                                                                                        as_str(): "__eq(a, b)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 582,
                                                                                                    end: 583,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 582,
                                                                                            end: 583,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 585,
                                                                                                    end: 586,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 585,
                                                                                            end: 586,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 577,
                                                                            end: 587,
                                                                            as_str(): "__eq(a, b)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 594,
                                                                                                        end: 596,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 594,
                                                                                                        end: 596,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 594,
                                                                                                    end: 596,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 594,
                                                                                        end: 596,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 592,
                                                                                                    end: 593,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 592,
                                                                                            end: 593,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 597,
                                                                                                    end: 598,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 597,
                                                                                            end: 598,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 592,
                                                                            end: 598,
                                                                            as_str(): "a == b",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 577,
                                                            end: 599,
                                                            as_str(): "__eq(a, b) == (a == b)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 570,
                                            end: 600,
                                            as_str(): "assert(__eq(a, b) == (a == b))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 570,
                                    end: 600,
                                    as_str(): "assert(__eq(a, b) == (a == b))",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 604,
                                                                end: 610,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 604,
                                                        end: 610,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 622,
                                                                                        end: 624,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 622,
                                                                                        end: 624,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 622,
                                                                                    end: 624,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 622,
                                                                        end: 624,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 611,
                                                                                        end: 615,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 611,
                                                                                        end: 621,
                                                                                        as_str(): "__eq(a, c)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 616,
                                                                                                    end: 617,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 616,
                                                                                            end: 617,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 619,
                                                                                                    end: 620,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 619,
                                                                                            end: 620,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 611,
                                                                            end: 621,
                                                                            as_str(): "__eq(a, c)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 628,
                                                                                                        end: 630,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 628,
                                                                                                        end: 630,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 628,
                                                                                                    end: 630,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 628,
                                                                                        end: 630,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 626,
                                                                                                    end: 627,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 626,
                                                                                            end: 627,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 631,
                                                                                                    end: 632,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 631,
                                                                                            end: 632,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 626,
                                                                            end: 632,
                                                                            as_str(): "a == c",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 611,
                                                            end: 633,
                                                            as_str(): "__eq(a, c) == (a == c)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 604,
                                            end: 634,
                                            as_str(): "assert(__eq(a, c) == (a == c))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 604,
                                    end: 634,
                                    as_str(): "assert(__eq(a, c) == (a == c))",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 643,
                                                    end: 644,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 646,
                                                    end: 649,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 652,
                                                    end: 653,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 639,
                                    end: 654,
                                    as_str(): "let a: u64 = 1;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 661,
                                                    end: 662,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 664,
                                                    end: 667,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        22,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 670,
                                                    end: 672,
                                                    as_str(): "22",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 657,
                                    end: 673,
                                    as_str(): "let b: u64 = 22;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 680,
                                                    end: 681,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 683,
                                                    end: 686,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                    ),
                                                    start: 689,
                                                    end: 690,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 676,
                                    end: 691,
                                    as_str(): "let c: u64 = 1;",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 694,
                                                                end: 700,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 694,
                                                        end: 700,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 712,
                                                                                        end: 714,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 712,
                                                                                        end: 714,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 712,
                                                                                    end: 714,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 712,
                                                                        end: 714,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 701,
                                                                                        end: 705,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 701,
                                                                                        end: 711,
                                                                                        as_str(): "__eq(a, b)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 706,
                                                                                                    end: 707,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 706,
                                                                                            end: 707,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 709,
                                                                                                    end: 710,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 709,
                                                                                            end: 710,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 701,
                                                                            end: 711,
                                                                            as_str(): "__eq(a, b)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 718,
                                                                                                        end: 720,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 718,
                                                                                                        end: 720,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 718,
                                                                                                    end: 720,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 718,
                                                                                        end: 720,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 716,
                                                                                                    end: 717,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 716,
                                                                                            end: 717,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 721,
                                                                                                    end: 722,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 721,
                                                                                            end: 722,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 716,
                                                                            end: 722,
                                                                            as_str(): "a == b",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 701,
                                                            end: 723,
                                                            as_str(): "__eq(a, b) == (a == b)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 694,
                                            end: 724,
                                            as_str(): "assert(__eq(a, b) == (a == b))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 694,
                                    end: 724,
                                    as_str(): "assert(__eq(a, b) == (a == b))",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 728,
                                                                end: 734,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0e931aa40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                        ),
                                                        start: 728,
                                                        end: 734,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 746,
                                                                                        end: 748,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 746,
                                                                                        end: 748,
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
                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 746,
                                                                                    end: 748,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 746,
                                                                        end: 748,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 735,
                                                                                        end: 739,
                                                                                        as_str(): "__eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: Eq,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 735,
                                                                                        end: 745,
                                                                                        as_str(): "__eq(a, c)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 740,
                                                                                                    end: 741,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 740,
                                                                                            end: 741,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 743,
                                                                                                    end: 744,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 743,
                                                                                            end: 744,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 735,
                                                                            end: 745,
                                                                            as_str(): "__eq(a, c)",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 752,
                                                                                                        end: 754,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 752,
                                                                                                        end: 754,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 752,
                                                                                                    end: 754,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 752,
                                                                                        end: 754,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 750,
                                                                                                    end: 751,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 750,
                                                                                            end: 751,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 755,
                                                                                                    end: 756,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 755,
                                                                                            end: 756,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 750,
                                                                            end: 756,
                                                                            as_str(): "a == c",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 735,
                                                            end: 757,
                                                            as_str(): "__eq(a, c) == (a == c)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 728,
                                            end: 758,
                                            as_str(): "assert(__eq(a, c) == (a == c))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 728,
                                    end: 758,
                                    as_str(): "assert(__eq(a, c) == (a == c))",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                2,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 763,
                                            end: 764,
                                            as_str(): "2",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0e931aa40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                    ),
                                    start: 763,
                                    end: 764,
                                    as_str(): "2",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 52,
                            end: 766,
                            as_str(): "{\n\n  assert(__eq(true, true) == (true == true));\n  assert(__eq(true, false) != (true != false));\n  assert(__eq(true, true) == (true != false));\n\n  assert(__eq(1, 22) == (1 == 22));\n  assert(__eq(1, 1) == (1 == 1));\n\n  let a: u8 = 1;\n  let b: u8 = 22;\n  let c: u8 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u16 = 1;\n  let b: u16 = 22;\n  let c: u16 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u32 = 1;\n  let b: u32 = 22;\n  let c: u32 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u64 = 1;\n  let b: u64 = 22;\n  let c: u64 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  2\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0e931aa40,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                        ),
                        start: 35,
                        end: 766,
                        as_str(): "fn main() -> u64 {\n\n  assert(__eq(true, true) == (true == true));\n  assert(__eq(true, false) != (true != false));\n  assert(__eq(true, true) == (true != false));\n\n  assert(__eq(1, 22) == (1 == 22));\n  assert(__eq(1, 1) == (1 == 1));\n\n  let a: u8 = 1;\n  let b: u8 = 22;\n  let c: u8 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u16 = 1;\n  let b: u16 = 22;\n  let c: u16 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u32 = 1;\n  let b: u32 = 22;\n  let c: u32 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u64 = 1;\n  let b: u64 = 22;\n  let c: u64 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  2\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0e931aa40,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                        ),
                        start: 48,
                        end: 51,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0e931aa40,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
            ),
            start: 35,
            end: 766,
            as_str(): "fn main() -> u64 {\n\n  assert(__eq(true, true) == (true == true));\n  assert(__eq(true, false) != (true != false));\n  assert(__eq(true, true) == (true != false));\n\n  assert(__eq(1, 22) == (1 == 22));\n  assert(__eq(1, 1) == (1 == 1));\n\n  let a: u8 = 1;\n  let b: u8 = 22;\n  let c: u8 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u16 = 1;\n  let b: u16 = 22;\n  let c: u16 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u32 = 1;\n  let b: u32 = 22;\n  let c: u32 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u64 = 1;\n  let b: u64 = 22;\n  let c: u64 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  2\n}",
        },
    },
]
