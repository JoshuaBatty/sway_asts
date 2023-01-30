[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
            src (ptr): 0x00007fe0503ea450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 66,
                                                    end: 67,
                                                    as_str(): "a",
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
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 70,
                                                    end: 71,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 58,
                                    end: 72,
                                    as_str(): "let mut a = 0;",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 82,
                                                                    end: 83,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 82,
                                                            end: 83,
                                                            as_str(): "a",
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 84,
                                                                                    end: 86,
                                                                                    as_str(): "+=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 84,
                                                                                    end: 86,
                                                                                    as_str(): "+=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "add",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 84,
                                                                                end: 86,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 84,
                                                                    end: 86,
                                                                    as_str(): "+=",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 82,
                                                                                end: 83,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 82,
                                                                        end: 83,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            99,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 87,
                                                                        end: 89,
                                                                        as_str(): "99",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 82,
                                                        end: 89,
                                                        as_str(): "a += 99",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 82,
                                            end: 89,
                                            as_str(): "a += 99",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 82,
                                    end: 89,
                                    as_str(): "a += 99",
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 95,
                                                                end: 101,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 95,
                                                        end: 101,
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
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 104,
                                                                        end: 106,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 102,
                                                                                    end: 103,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 103,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                99,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 107,
                                                                            end: 109,
                                                                            as_str(): "99",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 102,
                                                            end: 109,
                                                            as_str(): "a == 99",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 95,
                                            end: 110,
                                            as_str(): "assert(a == 99)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 110,
                                    as_str(): "assert(a == 99)",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 118,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 117,
                                                            end: 118,
                                                            as_str(): "a",
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 119,
                                                                                    end: 121,
                                                                                    as_str(): "-=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 119,
                                                                                    end: 121,
                                                                                    as_str(): "-=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "subtract",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 119,
                                                                                end: 121,
                                                                                as_str(): "-=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 119,
                                                                    end: 121,
                                                                    as_str(): "-=",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 117,
                                                                                end: 118,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 118,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 122,
                                                                        end: 123,
                                                                        as_str(): "5",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 117,
                                                        end: 123,
                                                        as_str(): "a -= 5",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 123,
                                            as_str(): "a -= 5",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 117,
                                    end: 123,
                                    as_str(): "a -= 5",
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 135,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 129,
                                                        end: 135,
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
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 138,
                                                                                        end: 140,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 138,
                                                                                        end: 140,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 138,
                                                                                    end: 140,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 138,
                                                                        end: 140,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 136,
                                                                                    end: 137,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 136,
                                                                            end: 137,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                94,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 141,
                                                                            end: 143,
                                                                            as_str(): "94",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 136,
                                                            end: 143,
                                                            as_str(): "a == 94",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 129,
                                            end: 144,
                                            as_str(): "assert(a == 94)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 129,
                                    end: 144,
                                    as_str(): "assert(a == 94)",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 151,
                                                                    end: 152,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 151,
                                                            end: 152,
                                                            as_str(): "a",
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 153,
                                                                                    end: 155,
                                                                                    as_str(): "*=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 153,
                                                                                    end: 155,
                                                                                    as_str(): "*=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "multiply",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 153,
                                                                                end: 155,
                                                                                as_str(): "*=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 153,
                                                                    end: 155,
                                                                    as_str(): "*=",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 151,
                                                                                end: 152,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 151,
                                                                        end: 152,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 156,
                                                                        end: 157,
                                                                        as_str(): "2",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 151,
                                                        end: 157,
                                                        as_str(): "a *= 2",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 151,
                                            end: 157,
                                            as_str(): "a *= 2",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 151,
                                    end: 157,
                                    as_str(): "a *= 2",
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 163,
                                                                end: 169,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 163,
                                                        end: 169,
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
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 174,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 174,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 174,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 174,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 170,
                                                                                    end: 171,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 170,
                                                                            end: 171,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                188,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 175,
                                                                            end: 178,
                                                                            as_str(): "188",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 178,
                                                            as_str(): "a == 188",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 163,
                                            end: 179,
                                            as_str(): "assert(a == 188)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 163,
                                    end: 179,
                                    as_str(): "assert(a == 188)",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 186,
                                                                    end: 187,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 186,
                                                            end: 187,
                                                            as_str(): "a",
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 188,
                                                                                    end: 190,
                                                                                    as_str(): "/=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 188,
                                                                                    end: 190,
                                                                                    as_str(): "/=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "divide",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 188,
                                                                                end: 190,
                                                                                as_str(): "/=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 188,
                                                                    end: 190,
                                                                    as_str(): "/=",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 186,
                                                                                end: 187,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 186,
                                                                        end: 187,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            47,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 191,
                                                                        end: 193,
                                                                        as_str(): "47",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 186,
                                                        end: 193,
                                                        as_str(): "a /= 47",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 193,
                                            as_str(): "a /= 47",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 186,
                                    end: 193,
                                    as_str(): "a /= 47",
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 208,
                                                                                        end: 210,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 208,
                                                                                        end: 210,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 208,
                                                                                    end: 210,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 208,
                                                                        end: 210,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 206,
                                                                                    end: 207,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 206,
                                                                            end: 207,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 211,
                                                                            end: 212,
                                                                            as_str(): "4",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 212,
                                                            as_str(): "a == 4",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 199,
                                            end: 213,
                                            as_str(): "assert(a == 4)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 199,
                                    end: 213,
                                    as_str(): "assert(a == 4)",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 220,
                                                                    end: 221,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 220,
                                                            end: 221,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            999,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 224,
                                                        end: 227,
                                                        as_str(): "999",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 220,
                                            end: 227,
                                            as_str(): "a = 999",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 220,
                                    end: 227,
                                    as_str(): "a = 999",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 234,
                                                                    end: 235,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 234,
                                                            end: 235,
                                                            as_str(): "a",
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 236,
                                                                                    end: 239,
                                                                                    as_str(): ">>=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 236,
                                                                                    end: 239,
                                                                                    as_str(): ">>=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "rsh",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 236,
                                                                                end: 239,
                                                                                as_str(): ">>=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 236,
                                                                    end: 239,
                                                                    as_str(): ">>=",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 234,
                                                                                end: 235,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 234,
                                                                        end: 235,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 240,
                                                                        end: 241,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 234,
                                                        end: 241,
                                                        as_str(): "a >>= 1",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 234,
                                            end: 241,
                                            as_str(): "a >>= 1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 234,
                                    end: 241,
                                    as_str(): "a >>= 1",
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 247,
                                                                end: 253,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 247,
                                                        end: 253,
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
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 256,
                                                                                        end: 258,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 256,
                                                                                        end: 258,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 256,
                                                                                    end: 258,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 256,
                                                                        end: 258,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 254,
                                                                                    end: 255,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 254,
                                                                            end: 255,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                499,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 262,
                                                                            as_str(): "499",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 254,
                                                            end: 262,
                                                            as_str(): "a == 499",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 247,
                                            end: 263,
                                            as_str(): "assert(a == 499)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 247,
                                    end: 263,
                                    as_str(): "assert(a == 499)",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 270,
                                                                    end: 271,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 270,
                                                            end: 271,
                                                            as_str(): "a",
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 272,
                                                                                    end: 275,
                                                                                    as_str(): "<<=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 272,
                                                                                    end: 275,
                                                                                    as_str(): "<<=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "lsh",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 272,
                                                                                end: 275,
                                                                                as_str(): "<<=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 272,
                                                                    end: 275,
                                                                    as_str(): "<<=",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0503ea450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                ),
                                                                                start: 270,
                                                                                end: 271,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 270,
                                                                        end: 271,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 276,
                                                                        end: 277,
                                                                        as_str(): "2",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 270,
                                                        end: 277,
                                                        as_str(): "a <<= 2",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 270,
                                            end: 277,
                                            as_str(): "a <<= 2",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 270,
                                    end: 277,
                                    as_str(): "a <<= 2",
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 283,
                                                                end: 289,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0503ea450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                        ),
                                                        start: 283,
                                                        end: 289,
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
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 292,
                                                                                        end: 294,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 292,
                                                                                        end: 294,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 292,
                                                                                    end: 294,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 292,
                                                                        end: 294,
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
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 290,
                                                                                    end: 291,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 290,
                                                                            end: 291,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1996,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 295,
                                                                            end: 299,
                                                                            as_str(): "1996",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 290,
                                                            end: 299,
                                                            as_str(): "a == 1996",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 283,
                                            end: 300,
                                            as_str(): "assert(a == 1996)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 283,
                                    end: 300,
                                    as_str(): "assert(a == 1996)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 307,
                                            end: 308,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0503ea450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                    ),
                                    start: 307,
                                    end: 308,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 52,
                            end: 310,
                            as_str(): "{\n    let mut a = 0;\n    \n    a += 99;\n    assert(a == 99);\n\n    a -= 5;\n    assert(a == 94);\n\n    a *= 2;\n    assert(a == 188);\n\n    a /= 47;\n    assert(a == 4);\n\n    a = 999;\n\n    a >>= 1;\n    assert(a == 499);\n\n    a <<= 2;\n    assert(a == 1996);\n\n    1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0503ea450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                        ),
                        start: 35,
                        end: 310,
                        as_str(): "fn main() -> u64 {\n    let mut a = 0;\n    \n    a += 99;\n    assert(a == 99);\n\n    a -= 5;\n    assert(a == 94);\n\n    a *= 2;\n    assert(a == 188);\n\n    a /= 47;\n    assert(a == 4);\n\n    a = 999;\n\n    a >>= 1;\n    assert(a == 499);\n\n    a <<= 2;\n    assert(a == 1996);\n\n    1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0503ea450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                        ),
                        start: 48,
                        end: 51,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0503ea450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
            ),
            start: 35,
            end: 310,
            as_str(): "fn main() -> u64 {\n    let mut a = 0;\n    \n    a += 99;\n    assert(a == 99);\n\n    a -= 5;\n    assert(a == 94);\n\n    a *= 2;\n    assert(a == 188);\n\n    a /= 47;\n    assert(a == 4);\n\n    a = 999;\n\n    a >>= 1;\n    assert(a == 499);\n\n    a <<= 2;\n    assert(a == 1996);\n\n    1\n}",
        },
    },
]
