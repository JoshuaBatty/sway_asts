[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 12,
                            end: 16,
                            as_str(): "core",
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
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 8,
            end: 20,
            as_str(): "use core::*;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
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
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
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
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
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
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 21,
            end: 45,
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
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 50,
                            end: 58,
                            as_str(): "sum_test",
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
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 97,
                                                    end: 100,
                                                    as_str(): "sum",
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 109,
                                                                            end: 110,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 109,
                                                                end: 110,
                                                                as_str(): "+",
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
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 105,
                                                                                                end: 106,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 105,
                                                                                                end: 106,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "add",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 105,
                                                                                            end: 106,
                                                                                            as_str(): "+",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 105,
                                                                                end: 106,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 103,
                                                                                            end: 104,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 104,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 107,
                                                                                            end: 108,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 107,
                                                                                    end: 108,
                                                                                    as_str(): "b",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 103,
                                                                    end: 108,
                                                                    as_str(): "a + b",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 111,
                                                                            end: 112,
                                                                            as_str(): "c",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 111,
                                                                    end: 112,
                                                                    as_str(): "c",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 103,
                                                    end: 112,
                                                    as_str(): "a + b + c",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 93,
                                    end: 113,
                                    as_str(): "let sum = a + b + c;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 118,
                                                    end: 121,
                                                    as_str(): "sum",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 118,
                                            end: 121,
                                            as_str(): "sum",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 118,
                                    end: 121,
                                    as_str(): "sum",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 87,
                            end: 123,
                            as_str(): "{\n    let sum = a + b + c;\n    sum\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 60,
                                    as_str(): "a",
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe073386450,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                ),
                                start: 61,
                                end: 64,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 66,
                                    end: 67,
                                    as_str(): "b",
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe073386450,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                ),
                                start: 68,
                                end: 71,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 73,
                                    end: 74,
                                    as_str(): "c",
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe073386450,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                ),
                                start: 75,
                                end: 78,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                        ),
                        start: 47,
                        end: 123,
                        as_str(): "fn sum_test(a:u64, b:u64, c:u64) -> u64 {\n    let sum = a + b + c;\n    sum\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                        ),
                        start: 83,
                        end: 86,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 47,
            end: 123,
            as_str(): "fn sum_test(a:u64, b:u64, c:u64) -> u64 {\n    let sum = a + b + c;\n    sum\n}",
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
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 128,
                            end: 145,
                            as_str(): "reassignment_test",
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
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 179,
                                                    end: 180,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        2,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 183,
                                                    end: 184,
                                                    as_str(): "2",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 171,
                                    end: 185,
                                    as_str(): "let mut b = 2;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 193,
                                                                end: 197,
                                                                as_str(): "cond",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 193,
                                                        end: 197,
                                                        as_str(): "cond",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Reassignment(
                                                                                ReassignmentExpression {
                                                                                    lhs: VariableExpression(
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe073386450,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 208,
                                                                                                        end: 209,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 208,
                                                                                                end: 209,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    rhs: Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                42,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 212,
                                                                                            end: 214,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 208,
                                                                                end: 214,
                                                                                as_str(): "b = 42",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 208,
                                                                        end: 214,
                                                                        as_str(): "b = 42",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 198,
                                                                end: 221,
                                                                as_str(): "{\n        b = 42;\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 198,
                                                        end: 221,
                                                        as_str(): "{\n        b = 42;\n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: Expression(
                                                                            Expression {
                                                                                kind: Reassignment(
                                                                                    ReassignmentExpression {
                                                                                        lhs: VariableExpression(
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe073386450,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                            ),
                                                                                                            start: 237,
                                                                                                            end: 238,
                                                                                                            as_str(): "b",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 237,
                                                                                                    end: 238,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        rhs: Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    5,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 241,
                                                                                                end: 242,
                                                                                                as_str(): "5",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 237,
                                                                                    end: 242,
                                                                                    as_str(): "b = 5",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 237,
                                                                            end: 242,
                                                                            as_str(): "b = 5",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 227,
                                                                    end: 249,
                                                                    as_str(): "{\n        b = 5;\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 227,
                                                            end: 249,
                                                            as_str(): "{\n        b = 5;\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 190,
                                            end: 249,
                                            as_str(): "if cond {\n        b = 42;\n    } else {\n        b = 5;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 190,
                                    end: 249,
                                    as_str(): "if cond {\n        b = 42;\n    } else {\n        b = 5;\n    }",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 255,
                                                    end: 256,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 255,
                                            end: 256,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 255,
                                    end: 256,
                                    as_str(): "b",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 165,
                            end: 258,
                            as_str(): "{\n    let mut b = 2;\n    if cond {\n        b = 42;\n    } else {\n        b = 5;\n    };\n    b\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 146,
                                    end: 150,
                                    as_str(): "cond",
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
                            type_info: Boolean,
                            type_span: Span {
                                src (ptr): 0x00007fe073386450,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                ),
                                start: 152,
                                end: 156,
                                as_str(): "bool",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                        ),
                        start: 125,
                        end: 258,
                        as_str(): "fn reassignment_test(cond: bool) -> u64 {\n    let mut b = 2;\n    if cond {\n        b = 42;\n    } else {\n        b = 5;\n    };\n    b\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                        ),
                        start: 161,
                        end: 164,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 125,
            end: 258,
            as_str(): "fn reassignment_test(cond: bool) -> u64 {\n    let mut b = 2;\n    if cond {\n        b = 42;\n    } else {\n        b = 5;\n    };\n    b\n}",
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
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 263,
                            end: 272,
                            as_str(): "loop_test",
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
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 310,
                                                    end: 311,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 314,
                                                    end: 315,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 302,
                                    end: 316,
                                    as_str(): "let mut b = 0;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: WhileLoop(
                                            WhileLoopExpression {
                                                condition: Expression {
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 329,
                                                                                    end: 330,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 329,
                                                                                    end: 330,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "lt",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 329,
                                                                                end: 330,
                                                                                as_str(): "<",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 329,
                                                                    end: 330,
                                                                    as_str(): "<",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 327,
                                                                                end: 328,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 327,
                                                                        end: 328,
                                                                        as_str(): "b",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 331,
                                                                                end: 341,
                                                                                as_str(): "trip_count",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 331,
                                                                        end: 341,
                                                                        as_str(): "trip_count",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 327,
                                                        end: 341,
                                                        as_str(): "b < trip_count",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
                                                        AstNode {
                                                            content: Expression(
                                                                Expression {
                                                                    kind: Reassignment(
                                                                        ReassignmentExpression {
                                                                            lhs: VariableExpression(
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 352,
                                                                                                end: 353,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 352,
                                                                                        end: 353,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            rhs: Expression {
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
                                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                                ),
                                                                                                                start: 358,
                                                                                                                end: 359,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                                ),
                                                                                                                start: 358,
                                                                                                                end: 359,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe073386450,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                            ),
                                                                                                            start: 358,
                                                                                                            end: 359,
                                                                                                            as_str(): "+",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 358,
                                                                                                end: 359,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe073386450,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                            ),
                                                                                                            start: 356,
                                                                                                            end: 357,
                                                                                                            as_str(): "b",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 356,
                                                                                                    end: 357,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 360,
                                                                                                    end: 361,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 356,
                                                                                    end: 361,
                                                                                    as_str(): "b + 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 352,
                                                                        end: 361,
                                                                        as_str(): "b = b + 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 352,
                                                                end: 361,
                                                                as_str(): "b = b + 1",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 342,
                                                        end: 368,
                                                        as_str(): "{\n        b = b + 1;\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 321,
                                            end: 368,
                                            as_str(): "while b < trip_count {\n        b = b + 1;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 321,
                                    end: 368,
                                    as_str(): "while b < trip_count {\n        b = b + 1;\n    }",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 375,
                                                                        end: 376,
                                                                        as_str(): "+",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "ops",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 375,
                                                                        end: 376,
                                                                        as_str(): "+",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            suffix: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "add",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 375,
                                                                    end: 376,
                                                                    as_str(): "+",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_absolute: true,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 375,
                                                        end: 376,
                                                        as_str(): "+",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 373,
                                                                    end: 374,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 373,
                                                            end: 374,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                1,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 377,
                                                            end: 378,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 373,
                                            end: 378,
                                            as_str(): "b + 1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 373,
                                    end: 378,
                                    as_str(): "b + 1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 296,
                            end: 380,
                            as_str(): "{\n    let mut b = 0;\n    while b < trip_count {\n        b = b + 1;\n    }\n    b + 1\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 273,
                                    end: 283,
                                    as_str(): "trip_count",
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe073386450,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                ),
                                start: 284,
                                end: 287,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                        ),
                        start: 260,
                        end: 380,
                        as_str(): "fn loop_test(trip_count:u64) -> u64 {\n    let mut b = 0;\n    while b < trip_count {\n        b = b + 1;\n    }\n    b + 1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                        ),
                        start: 292,
                        end: 295,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 260,
            end: 380,
            as_str(): "fn loop_test(trip_count:u64) -> u64 {\n    let mut b = 0;\n    while b < trip_count {\n        b = b + 1;\n    }\n    b + 1\n}",
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
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 385,
                            end: 389,
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
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 409,
                                                    end: 413,
                                                    as_str(): "four",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        4,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 416,
                                                    end: 417,
                                                    as_str(): "4",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 405,
                                    end: 418,
                                    as_str(): "let four = 4;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 427,
                                                    end: 432,
                                                    as_str(): "three",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        3,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 435,
                                                    end: 436,
                                                    as_str(): "3",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 423,
                                    end: 437,
                                    as_str(): "let three = 3;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 446,
                                                    end: 449,
                                                    as_str(): "sum",
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 457,
                                                                                end: 458,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 457,
                                                                                end: 458,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 457,
                                                                            end: 458,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 457,
                                                                end: 458,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 452,
                                                                            end: 456,
                                                                            as_str(): "four",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 452,
                                                                    end: 456,
                                                                    as_str(): "four",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 459,
                                                                            end: 464,
                                                                            as_str(): "three",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 459,
                                                                    end: 464,
                                                                    as_str(): "three",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 452,
                                                    end: 464,
                                                    as_str(): "four + three",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 442,
                                    end: 465,
                                    as_str(): "let sum = four + three;",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 470,
                                                                end: 476,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 470,
                                                        end: 476,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 481,
                                                                                        end: 483,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 481,
                                                                                        end: 483,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 481,
                                                                                    end: 483,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 481,
                                                                        end: 483,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 477,
                                                                                    end: 480,
                                                                                    as_str(): "sum",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 477,
                                                                            end: 480,
                                                                            as_str(): "sum",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 484,
                                                                            end: 485,
                                                                            as_str(): "7",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 477,
                                                            end: 485,
                                                            as_str(): "sum == 7",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 470,
                                            end: 486,
                                            as_str(): "assert(sum == 7)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 470,
                                    end: 486,
                                    as_str(): "assert(sum == 7)",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 493,
                                                                end: 499,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 493,
                                                        end: 499,
                                                        as_str(): "assert",
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
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 500,
                                                            end: 504,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 493,
                                            end: 505,
                                            as_str(): "assert(true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 493,
                                    end: 505,
                                    as_str(): "assert(true)",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 511,
                                                                end: 517,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 511,
                                                        end: 517,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 532,
                                                                                        end: 534,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 532,
                                                                                        end: 534,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 532,
                                                                                    end: 534,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 532,
                                                                        end: 534,
                                                                        as_str(): "==",
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
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 518,
                                                                                                end: 527,
                                                                                                as_str(): "loop_test",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 518,
                                                                                        end: 527,
                                                                                        as_str(): "loop_test",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                10,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 528,
                                                                                            end: 530,
                                                                                            as_str(): "10",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 518,
                                                                            end: 531,
                                                                            as_str(): "loop_test(10)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                11,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 535,
                                                                            end: 537,
                                                                            as_str(): "11",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 518,
                                                            end: 537,
                                                            as_str(): "loop_test(10) == 11",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 511,
                                            end: 538,
                                            as_str(): "assert(loop_test(10) == 11)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 511,
                                    end: 538,
                                    as_str(): "assert(loop_test(10) == 11)",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 544,
                                                                end: 550,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 544,
                                                        end: 550,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 576,
                                                                                        end: 578,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 576,
                                                                                        end: 578,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 576,
                                                                                    end: 578,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 576,
                                                                        end: 578,
                                                                        as_str(): "==",
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
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 551,
                                                                                                end: 568,
                                                                                                as_str(): "reassignment_test",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 551,
                                                                                        end: 568,
                                                                                        as_str(): "reassignment_test",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                false,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 569,
                                                                                            end: 574,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 551,
                                                                            end: 575,
                                                                            as_str(): "reassignment_test(false)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 579,
                                                                            end: 580,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 551,
                                                            end: 580,
                                                            as_str(): "reassignment_test(false) == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 544,
                                            end: 581,
                                            as_str(): "assert(reassignment_test(false) == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 544,
                                    end: 581,
                                    as_str(): "assert(reassignment_test(false) == 5)",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 587,
                                                                end: 593,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 587,
                                                        end: 593,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 612,
                                                                                        end: 614,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 612,
                                                                                        end: 614,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 612,
                                                                                    end: 614,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 612,
                                                                        end: 614,
                                                                        as_str(): "==",
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
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 594,
                                                                                                end: 602,
                                                                                                as_str(): "sum_test",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 594,
                                                                                        end: 602,
                                                                                        as_str(): "sum_test",
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
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 603,
                                                                                            end: 604,
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
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 606,
                                                                                            end: 607,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                3,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 609,
                                                                                            end: 610,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 594,
                                                                            end: 611,
                                                                            as_str(): "sum_test(1, 2, 3)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                6,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 615,
                                                                            end: 616,
                                                                            as_str(): "6",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 594,
                                                            end: 616,
                                                            as_str(): "sum_test(1, 2, 3) == 6",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 587,
                                            end: 617,
                                            as_str(): "assert(sum_test(1, 2, 3) == 6)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 587,
                                    end: 617,
                                    as_str(): "assert(sum_test(1, 2, 3) == 6)",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 623,
                                                                end: 629,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 623,
                                                        end: 629,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 651,
                                                                                        end: 653,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 651,
                                                                                        end: 653,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 651,
                                                                                    end: 653,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 651,
                                                                        end: 653,
                                                                        as_str(): "==",
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
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 630,
                                                                                                end: 638,
                                                                                                as_str(): "sum_test",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 630,
                                                                                        end: 638,
                                                                                        as_str(): "sum_test",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                30,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 639,
                                                                                            end: 641,
                                                                                            as_str(): "30",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                20,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 643,
                                                                                            end: 645,
                                                                                            as_str(): "20",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                10,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 647,
                                                                                            end: 649,
                                                                                            as_str(): "10",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 630,
                                                                            end: 650,
                                                                            as_str(): "sum_test(30, 20, 10)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                60,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 654,
                                                                            end: 656,
                                                                            as_str(): "60",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 630,
                                                            end: 656,
                                                            as_str(): "sum_test(30, 20, 10) == 60",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 623,
                                            end: 657,
                                            as_str(): "assert(sum_test(30, 20, 10) == 60)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 623,
                                    end: 657,
                                    as_str(): "assert(sum_test(30, 20, 10) == 60)",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 663,
                                                                end: 669,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 663,
                                                        end: 669,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 688,
                                                                                        end: 690,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 688,
                                                                                        end: 690,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 688,
                                                                                    end: 690,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 688,
                                                                        end: 690,
                                                                        as_str(): "==",
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
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 670,
                                                                                                end: 678,
                                                                                                as_str(): "sum_test",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 670,
                                                                                        end: 678,
                                                                                        as_str(): "sum_test",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                3,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 679,
                                                                                            end: 680,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 682,
                                                                                            end: 683,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 685,
                                                                                            end: 686,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 670,
                                                                            end: 687,
                                                                            as_str(): "sum_test(3, 2, 1)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                6,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 691,
                                                                            end: 692,
                                                                            as_str(): "6",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 670,
                                                            end: 692,
                                                            as_str(): "sum_test(3, 2, 1) == 6",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 663,
                                            end: 693,
                                            as_str(): "assert(sum_test(3, 2, 1) == 6)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 663,
                                    end: 693,
                                    as_str(): "assert(sum_test(3, 2, 1) == 6)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 704,
                                                    end: 707,
                                                    as_str(): "res",
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 710,
                                                                        end: 727,
                                                                        as_str(): "reassignment_test",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 710,
                                                                end: 727,
                                                                as_str(): "reassignment_test",
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
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 728,
                                                                    end: 732,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 710,
                                                    end: 733,
                                                    as_str(): "reassignment_test(true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 700,
                                    end: 734,
                                    as_str(): "let res = reassignment_test(true);",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 739,
                                                                end: 745,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 739,
                                                        end: 745,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 750,
                                                                                        end: 752,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 750,
                                                                                        end: 752,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 750,
                                                                                    end: 752,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 750,
                                                                        end: 752,
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
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 746,
                                                                                    end: 749,
                                                                                    as_str(): "res",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 746,
                                                                            end: 749,
                                                                            as_str(): "res",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 753,
                                                                            end: 755,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 746,
                                                            end: 755,
                                                            as_str(): "res == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 739,
                                            end: 756,
                                            as_str(): "assert(res == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 739,
                                    end: 756,
                                    as_str(): "assert(res == 42)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 762,
                                                    end: 765,
                                                    as_str(): "res",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 762,
                                            end: 765,
                                            as_str(): "res",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 762,
                                    end: 765,
                                    as_str(): "res",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 399,
                            end: 767,
                            as_str(): "{\n    let four = 4;\n    let three = 3;\n    let sum = four + three;\n    assert(sum == 7);\n\n    assert(true);\n    assert(loop_test(10) == 11);\n    assert(reassignment_test(false) == 5);\n    assert(sum_test(1, 2, 3) == 6);\n    assert(sum_test(30, 20, 10) == 60);\n    assert(sum_test(3, 2, 1) == 6);\n\n    let res = reassignment_test(true);\n    assert(res == 42);\n    res\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                        ),
                        start: 382,
                        end: 767,
                        as_str(): "fn main() -> u64 {\n    let four = 4;\n    let three = 3;\n    let sum = four + three;\n    assert(sum == 7);\n\n    assert(true);\n    assert(loop_test(10) == 11);\n    assert(reassignment_test(false) == 5);\n    assert(sum_test(1, 2, 3) == 6);\n    assert(sum_test(30, 20, 10) == 60);\n    assert(sum_test(3, 2, 1) == 6);\n\n    let res = reassignment_test(true);\n    assert(res == 42);\n    res\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                        ),
                        start: 395,
                        end: 398,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 382,
            end: 767,
            as_str(): "fn main() -> u64 {\n    let four = 4;\n    let three = 3;\n    let sum = four + three;\n    assert(sum == 7);\n\n    assert(true);\n    assert(loop_test(10) == 11);\n    assert(reassignment_test(false) == 5);\n    assert(sum_test(1, 2, 3) == 6);\n    assert(sum_test(30, 20, 10) == 60);\n    assert(sum_test(3, 2, 1) == 6);\n\n    let res = reassignment_test(true);\n    assert(res == 42);\n    res\n}",
        },
    },
]
