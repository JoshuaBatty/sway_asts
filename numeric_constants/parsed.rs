[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
            src (ptr): 0x00007fe06890a930,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 59,
                                                                end: 65,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 59,
                                                        end: 65,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 77,
                                                                                        end: 79,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 77,
                                                                                        end: 79,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 77,
                                                                                    end: 79,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 77,
                                                                        end: 79,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 66,
                                                                                                        end: 69,
                                                                                                        as_str(): "u64",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 66,
                                                                                                    end: 69,
                                                                                                    as_str(): "u64",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 71,
                                                                                                    end: 74,
                                                                                                    as_str(): "max",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 66,
                                                                                        end: 74,
                                                                                        as_str(): "u64::max",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 66,
                                                                            end: 76,
                                                                            as_str(): "u64::max()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                18446744073709551615,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 80,
                                                                            end: 100,
                                                                            as_str(): "18446744073709551615",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 66,
                                                            end: 100,
                                                            as_str(): "u64::max() == 18446744073709551615",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 101,
                                            as_str(): "assert(u64::max() == 18446744073709551615)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 101,
                                    as_str(): "assert(u64::max() == 18446744073709551615)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 107,
                                                                end: 113,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 107,
                                                        end: 113,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 125,
                                                                                        end: 127,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 125,
                                                                                        end: 127,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 125,
                                                                                    end: 127,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 125,
                                                                        end: 127,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 114,
                                                                                                        end: 117,
                                                                                                        as_str(): "u64",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 114,
                                                                                                    end: 117,
                                                                                                    as_str(): "u64",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 119,
                                                                                                    end: 122,
                                                                                                    as_str(): "min",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 114,
                                                                                        end: 122,
                                                                                        as_str(): "u64::min",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 114,
                                                                            end: 124,
                                                                            as_str(): "u64::min()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U64(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 128,
                                                                            end: 132,
                                                                            as_str(): "0u64",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 132,
                                                            as_str(): "u64::min() == 0u64",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 107,
                                            end: 133,
                                            as_str(): "assert(u64::min() == 0u64)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 107,
                                    end: 133,
                                    as_str(): "assert(u64::min() == 0u64)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 139,
                                                                end: 145,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 139,
                                                        end: 145,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 158,
                                                                                        end: 160,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 158,
                                                                                        end: 160,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 158,
                                                                                    end: 160,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 158,
                                                                        end: 160,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 146,
                                                                                                        end: 149,
                                                                                                        as_str(): "u64",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 146,
                                                                                                    end: 149,
                                                                                                    as_str(): "u64",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 151,
                                                                                                    end: 155,
                                                                                                    as_str(): "bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 146,
                                                                                        end: 155,
                                                                                        as_str(): "u64::bits",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 146,
                                                                            end: 157,
                                                                            as_str(): "u64::bits()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U32(
                                                                                64,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 161,
                                                                            end: 166,
                                                                            as_str(): "64u32",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 166,
                                                            as_str(): "u64::bits() == 64u32",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 139,
                                            end: 167,
                                            as_str(): "assert(u64::bits() == 64u32)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 139,
                                    end: 167,
                                    as_str(): "assert(u64::bits() == 64u32)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 173,
                                                                end: 179,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 173,
                                                        end: 179,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 191,
                                                                                        end: 193,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 191,
                                                                                        end: 193,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 191,
                                                                                    end: 193,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 191,
                                                                        end: 193,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 180,
                                                                                                        end: 183,
                                                                                                        as_str(): "u32",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 180,
                                                                                                    end: 183,
                                                                                                    as_str(): "u32",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 185,
                                                                                                    end: 188,
                                                                                                    as_str(): "max",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 180,
                                                                                        end: 188,
                                                                                        as_str(): "u32::max",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 180,
                                                                            end: 190,
                                                                            as_str(): "u32::max()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U32(
                                                                                4294967295,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 207,
                                                                            as_str(): "4294967295u32",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 180,
                                                            end: 207,
                                                            as_str(): "u32::max() == 4294967295u32",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 173,
                                            end: 208,
                                            as_str(): "assert(u32::max() == 4294967295u32)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 173,
                                    end: 208,
                                    as_str(): "assert(u32::max() == 4294967295u32)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 214,
                                                                end: 220,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 214,
                                                        end: 220,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 232,
                                                                                        end: 234,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 232,
                                                                                        end: 234,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 232,
                                                                                    end: 234,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 232,
                                                                        end: 234,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 221,
                                                                                                        end: 224,
                                                                                                        as_str(): "u32",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 221,
                                                                                                    end: 224,
                                                                                                    as_str(): "u32",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 226,
                                                                                                    end: 229,
                                                                                                    as_str(): "min",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 221,
                                                                                        end: 229,
                                                                                        as_str(): "u32::min",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 221,
                                                                            end: 231,
                                                                            as_str(): "u32::min()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U32(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 235,
                                                                            end: 239,
                                                                            as_str(): "0u32",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 239,
                                                            as_str(): "u32::min() == 0u32",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 214,
                                            end: 240,
                                            as_str(): "assert(u32::min() == 0u32)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 214,
                                    end: 240,
                                    as_str(): "assert(u32::min() == 0u32)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 246,
                                                                end: 252,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 246,
                                                        end: 252,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 265,
                                                                                        end: 267,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 265,
                                                                                        end: 267,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 265,
                                                                                    end: 267,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 265,
                                                                        end: 267,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 253,
                                                                                                        end: 256,
                                                                                                        as_str(): "u32",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 253,
                                                                                                    end: 256,
                                                                                                    as_str(): "u32",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 258,
                                                                                                    end: 262,
                                                                                                    as_str(): "bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 253,
                                                                                        end: 262,
                                                                                        as_str(): "u32::bits",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 253,
                                                                            end: 264,
                                                                            as_str(): "u32::bits()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U16(
                                                                                32,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 268,
                                                                            end: 273,
                                                                            as_str(): "32u16",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 273,
                                                            as_str(): "u32::bits() == 32u16",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 246,
                                            end: 274,
                                            as_str(): "assert(u32::bits() == 32u16)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 246,
                                    end: 274,
                                    as_str(): "assert(u32::bits() == 32u16)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 280,
                                                                end: 286,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 280,
                                                        end: 286,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 298,
                                                                                        end: 300,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 298,
                                                                                        end: 300,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 298,
                                                                                    end: 300,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 298,
                                                                        end: 300,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 287,
                                                                                                        end: 290,
                                                                                                        as_str(): "u16",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 287,
                                                                                                    end: 290,
                                                                                                    as_str(): "u16",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 292,
                                                                                                    end: 295,
                                                                                                    as_str(): "max",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 287,
                                                                                        end: 295,
                                                                                        as_str(): "u16::max",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 287,
                                                                            end: 297,
                                                                            as_str(): "u16::max()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U16(
                                                                                65535,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 301,
                                                                            end: 309,
                                                                            as_str(): "65535u16",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 287,
                                                            end: 309,
                                                            as_str(): "u16::max() == 65535u16",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 280,
                                            end: 310,
                                            as_str(): "assert(u16::max() == 65535u16)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 280,
                                    end: 310,
                                    as_str(): "assert(u16::max() == 65535u16)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 316,
                                                                end: 322,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 316,
                                                        end: 322,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 334,
                                                                                        end: 336,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 334,
                                                                                        end: 336,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 334,
                                                                                    end: 336,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 334,
                                                                        end: 336,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 323,
                                                                                                        end: 326,
                                                                                                        as_str(): "u16",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 323,
                                                                                                    end: 326,
                                                                                                    as_str(): "u16",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 328,
                                                                                                    end: 331,
                                                                                                    as_str(): "min",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 323,
                                                                                        end: 331,
                                                                                        as_str(): "u16::min",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 323,
                                                                            end: 333,
                                                                            as_str(): "u16::min()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U16(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 337,
                                                                            end: 341,
                                                                            as_str(): "0u16",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 341,
                                                            as_str(): "u16::min() == 0u16",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 316,
                                            end: 342,
                                            as_str(): "assert(u16::min() == 0u16)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 316,
                                    end: 342,
                                    as_str(): "assert(u16::min() == 0u16)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 348,
                                                                end: 354,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 348,
                                                        end: 354,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 367,
                                                                        end: 369,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 355,
                                                                                                        end: 358,
                                                                                                        as_str(): "u16",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 355,
                                                                                                    end: 358,
                                                                                                    as_str(): "u16",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 360,
                                                                                                    end: 364,
                                                                                                    as_str(): "bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 355,
                                                                                        end: 364,
                                                                                        as_str(): "u16::bits",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 355,
                                                                            end: 366,
                                                                            as_str(): "u16::bits()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U8(
                                                                                16,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 370,
                                                                            end: 374,
                                                                            as_str(): "16u8",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 355,
                                                            end: 374,
                                                            as_str(): "u16::bits() == 16u8",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 348,
                                            end: 375,
                                            as_str(): "assert(u16::bits() == 16u8)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 348,
                                    end: 375,
                                    as_str(): "assert(u16::bits() == 16u8)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 381,
                                                                end: 387,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 381,
                                                        end: 387,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 398,
                                                                                        end: 400,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 398,
                                                                                        end: 400,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 398,
                                                                                    end: 400,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 398,
                                                                        end: 400,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 388,
                                                                                                        end: 390,
                                                                                                        as_str(): "u8",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 388,
                                                                                                    end: 390,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 392,
                                                                                                    end: 395,
                                                                                                    as_str(): "max",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 388,
                                                                                        end: 395,
                                                                                        as_str(): "u8::max",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 388,
                                                                            end: 397,
                                                                            as_str(): "u8::max()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U8(
                                                                                255,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 401,
                                                                            end: 406,
                                                                            as_str(): "255u8",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 388,
                                                            end: 406,
                                                            as_str(): "u8::max() == 255u8",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 381,
                                            end: 407,
                                            as_str(): "assert(u8::max() == 255u8)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 381,
                                    end: 407,
                                    as_str(): "assert(u8::max() == 255u8)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 413,
                                                                end: 419,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 413,
                                                        end: 419,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 430,
                                                                                        end: 432,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 430,
                                                                                        end: 432,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 430,
                                                                                    end: 432,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 430,
                                                                        end: 432,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 420,
                                                                                                        end: 422,
                                                                                                        as_str(): "u8",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 420,
                                                                                                    end: 422,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 424,
                                                                                                    end: 427,
                                                                                                    as_str(): "min",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 420,
                                                                                        end: 427,
                                                                                        as_str(): "u8::min",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 420,
                                                                            end: 429,
                                                                            as_str(): "u8::min()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U8(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 433,
                                                                            end: 436,
                                                                            as_str(): "0u8",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 420,
                                                            end: 436,
                                                            as_str(): "u8::min() == 0u8",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 413,
                                            end: 437,
                                            as_str(): "assert(u8::min() == 0u8)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 413,
                                    end: 437,
                                    as_str(): "assert(u8::min() == 0u8)",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 443,
                                                                end: 449,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06890a930,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                        ),
                                                        start: 443,
                                                        end: 449,
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
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 461,
                                                                                        end: 463,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 461,
                                                                                        end: 463,
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
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 461,
                                                                                    end: 463,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 461,
                                                                        end: 463,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 450,
                                                                                                        end: 452,
                                                                                                        as_str(): "u8",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 450,
                                                                                                    end: 452,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 454,
                                                                                                    end: 458,
                                                                                                    as_str(): "bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06890a930,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 450,
                                                                                        end: 458,
                                                                                        as_str(): "u8::bits",
                                                                                    },
                                                                                },
                                                                                args: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 450,
                                                                            end: 460,
                                                                            as_str(): "u8::bits()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U32(
                                                                                8,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 464,
                                                                            end: 468,
                                                                            as_str(): "8u32",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 450,
                                                            end: 468,
                                                            as_str(): "u8::bits() == 8u32",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 443,
                                            end: 469,
                                            as_str(): "assert(u8::bits() == 8u32)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 443,
                                    end: 469,
                                    as_str(): "assert(u8::bits() == 8u32)",
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
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 476,
                                            end: 480,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06890a930,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                    ),
                                    start: 476,
                                    end: 480,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 53,
                            end: 482,
                            as_str(): "{\n    assert(u64::max() == 18446744073709551615);\n    assert(u64::min() == 0u64);\n    assert(u64::bits() == 64u32);\n    assert(u32::max() == 4294967295u32);\n    assert(u32::min() == 0u32);\n    assert(u32::bits() == 32u16);\n    assert(u16::max() == 65535u16);\n    assert(u16::min() == 0u16);\n    assert(u16::bits() == 16u8);\n    assert(u8::max() == 255u8);\n    assert(u8::min() == 0u8);\n    assert(u8::bits() == 8u32);\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe06890a930,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                        ),
                        start: 35,
                        end: 482,
                        as_str(): "fn main() -> bool {\n    assert(u64::max() == 18446744073709551615);\n    assert(u64::min() == 0u64);\n    assert(u64::bits() == 64u32);\n    assert(u32::max() == 4294967295u32);\n    assert(u32::min() == 0u32);\n    assert(u32::bits() == 32u16);\n    assert(u16::max() == 65535u16);\n    assert(u16::min() == 0u16);\n    assert(u16::bits() == 16u8);\n    assert(u8::max() == 255u8);\n    assert(u8::min() == 0u8);\n    assert(u8::bits() == 8u32);\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06890a930,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                        ),
                        start: 48,
                        end: 52,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06890a930,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
            ),
            start: 35,
            end: 482,
            as_str(): "fn main() -> bool {\n    assert(u64::max() == 18446744073709551615);\n    assert(u64::min() == 0u64);\n    assert(u64::bits() == 64u32);\n    assert(u32::max() == 4294967295u32);\n    assert(u32::min() == 0u32);\n    assert(u32::bits() == 32u16);\n    assert(u16::max() == 65535u16);\n    assert(u16::min() == 0u16);\n    assert(u16::bits() == 16u8);\n    assert(u8::max() == 255u8);\n    assert(u8::min() == 0u8);\n    assert(u8::bits() == 8u32);\n\n    true\n}",
        },
    },
]
