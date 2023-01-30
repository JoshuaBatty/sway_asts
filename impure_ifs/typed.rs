

TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 56,
            end: 60,
            as_str(): "Bool",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 67,
                    end: 71,
                    as_str(): "True",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31631,
            ),
            initial_type_id: TypeId(
                31630,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 73,
                end: 75,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 67,
                end: 75,
                as_str(): "True: ()",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 81,
                    end: 86,
                    as_str(): "False",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31633,
            ),
            initial_type_id: TypeId(
                31632,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 88,
                end: 90,
                as_str(): "()",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 81,
                end: 90,
                as_str(): "False: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 51,
        end: 93,
        as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 98,
            end: 101,
            as_str(): "foo",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 102,
                                            end: 103,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 127,
                                        end: 128,
                                        as_str(): "b",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 127,
                                    end: 128,
                                    as_str(): "b",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                101,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 139,
                                                            end: 142,
                                                            as_str(): "101",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 139,
                                                    end: 142,
                                                    as_str(): "101",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 129,
                                    end: 148,
                                    as_str(): "{\n        101\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    102,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 164,
                                                                end: 167,
                                                                as_str(): "102",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 164,
                                                        end: 167,
                                                        as_str(): "102",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 154,
                                        end: 173,
                                        as_str(): "{\n        102\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 124,
                            end: 173,
                            as_str(): "if b {\n        101\n    } else {\n        102\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 124,
                    end: 173,
                    as_str(): "if b {\n        101\n    } else {\n        102\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 102,
                    end: 103,
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
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 105,
                end: 109,
                as_str(): "bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 95,
        end: 175,
        as_str(): "fn foo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        102\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 114,
        end: 117,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 180,
            end: 183,
            as_str(): "bar",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 184,
                                            end: 185,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 209,
                                        end: 210,
                                        as_str(): "b",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 209,
                                    end: 210,
                                    as_str(): "b",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Return(
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        101,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 228,
                                                                    end: 231,
                                                                    as_str(): "101",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31644,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 231,
                                                            as_str(): "return 101",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 221,
                                                    end: 231,
                                                    as_str(): "return 101",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7215,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 211,
                                    end: 238,
                                    as_str(): "{\n        return 101;\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: Expression(
                                                        TyExpression {
                                                            expression: Return(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            102,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 261,
                                                                        end: 264,
                                                                        as_str(): "102",
                                                                    },
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31649,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 254,
                                                                end: 264,
                                                                as_str(): "return 102",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 254,
                                                        end: 264,
                                                        as_str(): "return 102",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7215,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 244,
                                        end: 271,
                                        as_str(): "{\n        return 102;\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7215,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 206,
                            end: 271,
                            as_str(): "if b {\n        return 101;\n    } else {\n        return 102;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 206,
                    end: 271,
                    as_str(): "if b {\n        return 101;\n    } else {\n        return 102;\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 184,
                    end: 185,
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
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 187,
                end: 191,
                as_str(): "bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 177,
        end: 273,
        as_str(): "fn bar(b: bool) -> u64 {\n    if b {\n        return 101;\n    } else {\n        return 102;\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 196,
        end: 199,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 278,
            end: 282,
            as_str(): "bell",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 283,
                                            end: 284,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 308,
                                        end: 309,
                                        as_str(): "b",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 308,
                                    end: 309,
                                    as_str(): "b",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Return(
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        101,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 327,
                                                                    end: 330,
                                                                    as_str(): "101",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31655,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 320,
                                                            end: 330,
                                                            as_str(): "return 101",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 320,
                                                    end: 330,
                                                    as_str(): "return 101",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7215,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 310,
                                    end: 337,
                                    as_str(): "{\n        return 101;\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    102,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 353,
                                                                end: 356,
                                                                as_str(): "102",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 353,
                                                        end: 356,
                                                        as_str(): "102",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 343,
                                        end: 362,
                                        as_str(): "{\n        102\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 305,
                            end: 362,
                            as_str(): "if b {\n        return 101;\n    } else {\n        102\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 305,
                    end: 362,
                    as_str(): "if b {\n        return 101;\n    } else {\n        102\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 283,
                    end: 284,
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
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 286,
                end: 290,
                as_str(): "bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 275,
        end: 364,
        as_str(): "fn bell(b: bool) -> u64 {\n    if b {\n        return 101;\n    } else {\n        102\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 295,
        end: 298,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 369,
            end: 372,
            as_str(): "moo",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 373,
                                            end: 374,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 398,
                                        end: 399,
                                        as_str(): "b",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 398,
                                    end: 399,
                                    as_str(): "b",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                101,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 410,
                                                            end: 413,
                                                            as_str(): "101",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 410,
                                                    end: 413,
                                                    as_str(): "101",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 400,
                                    end: 419,
                                    as_str(): "{\n        101\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: Expression(
                                                        TyExpression {
                                                            expression: Return(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            102,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 442,
                                                                        end: 445,
                                                                        as_str(): "102",
                                                                    },
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31665,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 435,
                                                                end: 445,
                                                                as_str(): "return 102",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 435,
                                                        end: 445,
                                                        as_str(): "return 102",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7215,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 425,
                                        end: 452,
                                        as_str(): "{\n        return 102;\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 395,
                            end: 452,
                            as_str(): "if b {\n        101\n    } else {\n        return 102;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 395,
                    end: 452,
                    as_str(): "if b {\n        101\n    } else {\n        return 102;\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 373,
                    end: 374,
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
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 376,
                end: 380,
                as_str(): "bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 366,
        end: 454,
        as_str(): "fn moo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        return 102;\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 385,
        end: 388,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 459,
            end: 462,
            as_str(): "poo",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 505,
                                                    end: 506,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 505,
                                                    end: 506,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                ),
                                                start: 505,
                                                end: 506,
                                                as_str(): "b",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3022,
                                                    end: 3026,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: EnumTag {
                                                    exp: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 463,
                                                                    end: 464,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 505,
                                                                end: 506,
                                                                as_str(): "b",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31668,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 505,
                                                            end: 506,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 505,
                                                    end: 506,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3028,
                                                    end: 3033,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 505,
                                                    end: 506,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13319,
                                        Span {
                                            src (ptr): 0x00007fe0fb532830,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3016,
                                            end: 3082,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: None,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 505,
                                    end: 506,
                                    as_str(): "b",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                101,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 517,
                                                            end: 520,
                                                            as_str(): "101",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 517,
                                                    end: 520,
                                                    as_str(): "101",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 507,
                                    end: 526,
                                    as_str(): "{\n        101\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: Expression(
                                                        TyExpression {
                                                            expression: Return(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            102,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 549,
                                                                        end: 552,
                                                                        as_str(): "102",
                                                                    },
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31680,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 542,
                                                                end: 552,
                                                                as_str(): "return 102",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 542,
                                                        end: 552,
                                                        as_str(): "return 102",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7215,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 532,
                                        end: 559,
                                        as_str(): "{\n        return 102;\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 507,
                            end: 526,
                            as_str(): "{\n        101\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 485,
                    end: 559,
                    as_str(): "if let Bool::True = b {\n        101\n    } else {\n        return 102;\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 463,
                    end: 464,
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
            type_id: TypeId(
                31668,
            ),
            initial_type_id: TypeId(
                31667,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 466,
                end: 470,
                as_str(): "Bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 456,
        end: 561,
        as_str(): "fn poo(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        return 102;\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 475,
        end: 478,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 566,
            end: 582,
            as_str(): "ran_out_of_names",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 625,
                                                    end: 626,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 625,
                                                    end: 626,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                ),
                                                start: 625,
                                                end: 626,
                                                as_str(): "b",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3022,
                                                    end: 3026,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: EnumTag {
                                                    exp: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 583,
                                                                    end: 584,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 625,
                                                                end: 626,
                                                                as_str(): "b",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31668,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 625,
                                                            end: 626,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 625,
                                                    end: 626,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3028,
                                                    end: 3033,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 625,
                                                    end: 626,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13321,
                                        Span {
                                            src (ptr): 0x00007fe0fb532830,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3016,
                                            end: 3082,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: None,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 625,
                                    end: 626,
                                    as_str(): "b",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Return(
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        101,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 644,
                                                                    end: 647,
                                                                    as_str(): "101",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31690,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 637,
                                                            end: 647,
                                                            as_str(): "return 101",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 637,
                                                    end: 647,
                                                    as_str(): "return 101",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7215,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 627,
                                    end: 654,
                                    as_str(): "{\n        return 101;\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: Expression(
                                                        TyExpression {
                                                            expression: Return(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            102,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 677,
                                                                        end: 680,
                                                                        as_str(): "102",
                                                                    },
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31697,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 670,
                                                                end: 680,
                                                                as_str(): "return 102",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 670,
                                                        end: 680,
                                                        as_str(): "return 102",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7215,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 660,
                                        end: 687,
                                        as_str(): "{\n        return 102;\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7215,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 627,
                            end: 654,
                            as_str(): "{\n        return 101;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 605,
                    end: 687,
                    as_str(): "if let Bool::True = b {\n        return 101;\n    } else {\n        return 102;\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 583,
                    end: 584,
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
            type_id: TypeId(
                31668,
            ),
            initial_type_id: TypeId(
                31682,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 586,
                end: 590,
                as_str(): "Bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 563,
        end: 689,
        as_str(): "fn ran_out_of_names(b: Bool) -> u64 {\n    if let Bool::True = b {\n        return 101;\n    } else {\n        return 102;\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 595,
        end: 598,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 694,
            end: 704,
            as_str(): "another_fn",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 747,
                                                    end: 748,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 747,
                                                    end: 748,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                ),
                                                start: 747,
                                                end: 748,
                                                as_str(): "b",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3022,
                                                    end: 3026,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: EnumTag {
                                                    exp: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 705,
                                                                    end: 706,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 747,
                                                                end: 748,
                                                                as_str(): "b",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31668,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 747,
                                                            end: 748,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 747,
                                                    end: 748,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3028,
                                                    end: 3033,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 747,
                                                    end: 748,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13323,
                                        Span {
                                            src (ptr): 0x00007fe0fb532830,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3016,
                                            end: 3082,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: None,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 747,
                                    end: 748,
                                    as_str(): "b",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Return(
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        101,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 766,
                                                                    end: 769,
                                                                    as_str(): "101",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31707,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 759,
                                                            end: 769,
                                                            as_str(): "return 101",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 759,
                                                    end: 769,
                                                    as_str(): "return 101",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7215,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 749,
                                    end: 776,
                                    as_str(): "{\n        return 101;\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    102,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 792,
                                                                end: 795,
                                                                as_str(): "102",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 792,
                                                        end: 795,
                                                        as_str(): "102",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 782,
                                        end: 801,
                                        as_str(): "{\n        102\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 749,
                            end: 776,
                            as_str(): "{\n        return 101;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 727,
                    end: 801,
                    as_str(): "if let Bool::True = b {\n        return 101;\n    } else {\n        102\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 705,
                    end: 706,
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
            type_id: TypeId(
                31668,
            ),
            initial_type_id: TypeId(
                31699,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 708,
                end: 712,
                as_str(): "Bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 691,
        end: 803,
        as_str(): "fn another_fn(b: Bool) -> u64 {\n    if let Bool::True = b {\n        return 101;\n    } else {\n        102\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 717,
        end: 720,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 808,
            end: 817,
            as_str(): "thats_all",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 860,
                                                    end: 861,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 860,
                                                    end: 861,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                ),
                                                start: 860,
                                                end: 861,
                                                as_str(): "b",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3022,
                                                    end: 3026,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: EnumTag {
                                                    exp: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 818,
                                                                    end: 819,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 860,
                                                                end: 861,
                                                                as_str(): "b",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31668,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 860,
                                                            end: 861,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 860,
                                                    end: 861,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3028,
                                                    end: 3033,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 860,
                                                    end: 861,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13325,
                                        Span {
                                            src (ptr): 0x00007fe0fb532830,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3016,
                                            end: 3082,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: None,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 860,
                                    end: 861,
                                    as_str(): "b",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                101,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 872,
                                                            end: 875,
                                                            as_str(): "101",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 872,
                                                    end: 875,
                                                    as_str(): "101",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0d2fa5ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                    ),
                                    start: 862,
                                    end: 881,
                                    as_str(): "{\n        101\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    102,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 897,
                                                                end: 900,
                                                                as_str(): "102",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 897,
                                                        end: 900,
                                                        as_str(): "102",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 887,
                                        end: 906,
                                        as_str(): "{\n        102\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 862,
                            end: 881,
                            as_str(): "{\n        101\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 840,
                    end: 906,
                    as_str(): "if let Bool::True = b {\n        101\n    } else {\n        102\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 818,
                    end: 819,
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
            type_id: TypeId(
                31668,
            ),
            initial_type_id: TypeId(
                31713,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0d2fa5ff0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                ),
                start: 821,
                end: 825,
                as_str(): "Bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 805,
        end: 908,
        as_str(): "fn thats_all(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        102\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 830,
        end: 833,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0d2fa5ff0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
            ),
            start: 913,
            end: 917,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 933,
                                        end: 939,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 950,
                                                            end: 952,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 950,
                                                            end: 952,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 950,
                                                        end: 952,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 940,
                                                                        end: 943,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 103,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 944,
                                                                            end: 948,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13329,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 175,
                                                                    as_str(): "fn foo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 940,
                                                                        end: 943,
                                                                        as_str(): "foo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 940,
                                                            end: 949,
                                                            as_str(): "foo(true)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 953,
                                                                        end: 956,
                                                                        as_str(): "bar",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 184,
                                                                            end: 185,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 957,
                                                                            end: 961,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13331,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 177,
                                                                    end: 273,
                                                                    as_str(): "fn bar(b: bool) -> u64 {\n    if b {\n        return 101;\n    } else {\n        return 102;\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 953,
                                                                        end: 956,
                                                                        as_str(): "bar",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 953,
                                                            end: 962,
                                                            as_str(): "bar(true)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13332,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 950,
                                                        end: 952,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 940,
                                            end: 962,
                                            as_str(): "foo(true) == bar(true)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 933,
                                        end: 939,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31730,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 933,
                            end: 963,
                            as_str(): "assert(foo(true) == bar(true))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 933,
                    end: 963,
                    as_str(): "assert(foo(true) == bar(true))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 969,
                                        end: 975,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 987,
                                                            end: 989,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 987,
                                                            end: 989,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 987,
                                                        end: 989,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 976,
                                                                        end: 979,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 103,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 980,
                                                                            end: 985,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13336,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 175,
                                                                    as_str(): "fn foo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 976,
                                                                        end: 979,
                                                                        as_str(): "foo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 976,
                                                            end: 986,
                                                            as_str(): "foo(false)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 990,
                                                                        end: 993,
                                                                        as_str(): "bar",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 184,
                                                                            end: 185,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 994,
                                                                            end: 999,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13338,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 177,
                                                                    end: 273,
                                                                    as_str(): "fn bar(b: bool) -> u64 {\n    if b {\n        return 101;\n    } else {\n        return 102;\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 990,
                                                                        end: 993,
                                                                        as_str(): "bar",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 990,
                                                            end: 1000,
                                                            as_str(): "bar(false)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13339,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 987,
                                                        end: 989,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 976,
                                            end: 1000,
                                            as_str(): "foo(false) == bar(false)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13340,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 969,
                                        end: 975,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31737,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 969,
                            end: 1001,
                            as_str(): "assert(foo(false) == bar(false))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 969,
                    end: 1001,
                    as_str(): "assert(foo(false) == bar(false))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1007,
                                        end: 1013,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1024,
                                                            end: 1026,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1024,
                                                            end: 1026,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1024,
                                                        end: 1026,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1014,
                                                                        end: 1017,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 103,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1018,
                                                                            end: 1022,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13343,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 175,
                                                                    as_str(): "fn foo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1014,
                                                                        end: 1017,
                                                                        as_str(): "foo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1014,
                                                            end: 1023,
                                                            as_str(): "foo(true)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1027,
                                                                        end: 1031,
                                                                        as_str(): "bell",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 283,
                                                                            end: 284,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1032,
                                                                            end: 1036,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13345,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 275,
                                                                    end: 364,
                                                                    as_str(): "fn bell(b: bool) -> u64 {\n    if b {\n        return 101;\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1027,
                                                                        end: 1031,
                                                                        as_str(): "bell",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1027,
                                                            end: 1037,
                                                            as_str(): "bell(true)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13346,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1024,
                                                        end: 1026,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1014,
                                            end: 1037,
                                            as_str(): "foo(true) == bell(true)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13347,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1007,
                                        end: 1013,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31744,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1007,
                            end: 1038,
                            as_str(): "assert(foo(true) == bell(true))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1007,
                    end: 1038,
                    as_str(): "assert(foo(true) == bell(true))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1044,
                                        end: 1050,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1062,
                                                            end: 1064,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1062,
                                                            end: 1064,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1062,
                                                        end: 1064,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1051,
                                                                        end: 1054,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 103,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1055,
                                                                            end: 1060,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13350,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 175,
                                                                    as_str(): "fn foo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1051,
                                                                        end: 1054,
                                                                        as_str(): "foo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1051,
                                                            end: 1061,
                                                            as_str(): "foo(false)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1065,
                                                                        end: 1069,
                                                                        as_str(): "bell",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 283,
                                                                            end: 284,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1070,
                                                                            end: 1075,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13352,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 275,
                                                                    end: 364,
                                                                    as_str(): "fn bell(b: bool) -> u64 {\n    if b {\n        return 101;\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1065,
                                                                        end: 1069,
                                                                        as_str(): "bell",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1065,
                                                            end: 1076,
                                                            as_str(): "bell(false)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13353,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1062,
                                                        end: 1064,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1051,
                                            end: 1076,
                                            as_str(): "foo(false) == bell(false)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13354,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1044,
                                        end: 1050,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31751,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1044,
                            end: 1077,
                            as_str(): "assert(foo(false) == bell(false))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1044,
                    end: 1077,
                    as_str(): "assert(foo(false) == bell(false))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1083,
                                        end: 1089,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1100,
                                                            end: 1102,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1100,
                                                            end: 1102,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1100,
                                                        end: 1102,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1090,
                                                                        end: 1093,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 103,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1094,
                                                                            end: 1098,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13357,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 175,
                                                                    as_str(): "fn foo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1090,
                                                                        end: 1093,
                                                                        as_str(): "foo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1090,
                                                            end: 1099,
                                                            as_str(): "foo(true)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1103,
                                                                        end: 1106,
                                                                        as_str(): "moo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 373,
                                                                            end: 374,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1107,
                                                                            end: 1111,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13359,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 366,
                                                                    end: 454,
                                                                    as_str(): "fn moo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        return 102;\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1103,
                                                                        end: 1106,
                                                                        as_str(): "moo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1103,
                                                            end: 1112,
                                                            as_str(): "moo(true)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13360,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1100,
                                                        end: 1102,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1090,
                                            end: 1112,
                                            as_str(): "foo(true) == moo(true)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1083,
                                        end: 1089,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31758,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1083,
                            end: 1113,
                            as_str(): "assert(foo(true) == moo(true))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1083,
                    end: 1113,
                    as_str(): "assert(foo(true) == moo(true))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1119,
                                        end: 1125,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1137,
                                                            end: 1139,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1137,
                                                            end: 1139,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1137,
                                                        end: 1139,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1126,
                                                                        end: 1129,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 103,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1130,
                                                                            end: 1135,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13364,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 175,
                                                                    as_str(): "fn foo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1126,
                                                                        end: 1129,
                                                                        as_str(): "foo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1126,
                                                            end: 1136,
                                                            as_str(): "foo(false)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1140,
                                                                        end: 1143,
                                                                        as_str(): "moo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 373,
                                                                            end: 374,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1144,
                                                                            end: 1149,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13366,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 366,
                                                                    end: 454,
                                                                    as_str(): "fn moo(b: bool) -> u64 {\n    if b {\n        101\n    } else {\n        return 102;\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1140,
                                                                        end: 1143,
                                                                        as_str(): "moo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1140,
                                                            end: 1150,
                                                            as_str(): "moo(false)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13367,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1137,
                                                        end: 1139,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1126,
                                            end: 1150,
                                            as_str(): "foo(false) == moo(false)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13368,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1119,
                                        end: 1125,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31765,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1119,
                            end: 1151,
                            as_str(): "assert(foo(false) == moo(false))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1119,
                    end: 1151,
                    as_str(): "assert(foo(false) == moo(false))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1158,
                                        end: 1164,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1187,
                                                            end: 1189,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1187,
                                                            end: 1189,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1187,
                                                        end: 1189,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1165,
                                                                        end: 1174,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 818,
                                                                            end: 819,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 67,
                                                                                    end: 71,
                                                                                    as_str(): "True",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 0,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1175,
                                                                                end: 1179,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1181,
                                                                                end: 1185,
                                                                                as_str(): "True",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1181,
                                                                                    end: 1185,
                                                                                    as_str(): "True",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1181,
                                                                            end: 1185,
                                                                            as_str(): "True",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13372,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 805,
                                                                    end: 908,
                                                                    as_str(): "fn thats_all(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1165,
                                                                        end: 1174,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1165,
                                                            end: 1186,
                                                            as_str(): "thats_all(Bool::True)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1190,
                                                                        end: 1193,
                                                                        as_str(): "poo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 463,
                                                                            end: 464,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 67,
                                                                                    end: 71,
                                                                                    as_str(): "True",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 0,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1194,
                                                                                end: 1198,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1200,
                                                                                end: 1204,
                                                                                as_str(): "True",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1200,
                                                                                    end: 1204,
                                                                                    as_str(): "True",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1200,
                                                                            end: 1204,
                                                                            as_str(): "True",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13375,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 456,
                                                                    end: 561,
                                                                    as_str(): "fn poo(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        return 102;\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1190,
                                                                        end: 1193,
                                                                        as_str(): "poo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1190,
                                                            end: 1205,
                                                            as_str(): "poo(Bool::True)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13376,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1187,
                                                        end: 1189,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1165,
                                            end: 1205,
                                            as_str(): "thats_all(Bool::True) == poo(Bool::True)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13377,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1158,
                                        end: 1164,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31772,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1158,
                            end: 1206,
                            as_str(): "assert(thats_all(Bool::True) == poo(Bool::True))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1158,
                    end: 1206,
                    as_str(): "assert(thats_all(Bool::True) == poo(Bool::True))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1212,
                                        end: 1218,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1242,
                                                            end: 1244,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1242,
                                                            end: 1244,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1242,
                                                        end: 1244,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1219,
                                                                        end: 1228,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 818,
                                                                            end: 819,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 86,
                                                                                    as_str(): "False",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 1,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1229,
                                                                                end: 1233,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1235,
                                                                                end: 1240,
                                                                                as_str(): "False",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1235,
                                                                                    end: 1240,
                                                                                    as_str(): "False",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1235,
                                                                            end: 1240,
                                                                            as_str(): "False",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13381,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 805,
                                                                    end: 908,
                                                                    as_str(): "fn thats_all(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1219,
                                                                        end: 1228,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1219,
                                                            end: 1241,
                                                            as_str(): "thats_all(Bool::False)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1245,
                                                                        end: 1248,
                                                                        as_str(): "poo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 463,
                                                                            end: 464,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 86,
                                                                                    as_str(): "False",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 1,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1249,
                                                                                end: 1253,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1255,
                                                                                end: 1260,
                                                                                as_str(): "False",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1255,
                                                                                    end: 1260,
                                                                                    as_str(): "False",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1255,
                                                                            end: 1260,
                                                                            as_str(): "False",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13384,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 456,
                                                                    end: 561,
                                                                    as_str(): "fn poo(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        return 102;\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1245,
                                                                        end: 1248,
                                                                        as_str(): "poo",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1245,
                                                            end: 1261,
                                                            as_str(): "poo(Bool::False)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13385,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1242,
                                                        end: 1244,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1219,
                                            end: 1261,
                                            as_str(): "thats_all(Bool::False) == poo(Bool::False)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13386,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1212,
                                        end: 1218,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31779,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1212,
                            end: 1262,
                            as_str(): "assert(thats_all(Bool::False) == poo(Bool::False))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1212,
                    end: 1262,
                    as_str(): "assert(thats_all(Bool::False) == poo(Bool::False))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1268,
                                        end: 1274,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1297,
                                                            end: 1299,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1297,
                                                            end: 1299,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1297,
                                                        end: 1299,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1275,
                                                                        end: 1284,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 818,
                                                                            end: 819,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 67,
                                                                                    end: 71,
                                                                                    as_str(): "True",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 0,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1285,
                                                                                end: 1289,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1291,
                                                                                end: 1295,
                                                                                as_str(): "True",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1291,
                                                                                    end: 1295,
                                                                                    as_str(): "True",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1291,
                                                                            end: 1295,
                                                                            as_str(): "True",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13390,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 805,
                                                                    end: 908,
                                                                    as_str(): "fn thats_all(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1275,
                                                                        end: 1284,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1275,
                                                            end: 1296,
                                                            as_str(): "thats_all(Bool::True)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1300,
                                                                        end: 1316,
                                                                        as_str(): "ran_out_of_names",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 583,
                                                                            end: 584,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 67,
                                                                                    end: 71,
                                                                                    as_str(): "True",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 0,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1317,
                                                                                end: 1321,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1323,
                                                                                end: 1327,
                                                                                as_str(): "True",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1323,
                                                                                    end: 1327,
                                                                                    as_str(): "True",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1323,
                                                                            end: 1327,
                                                                            as_str(): "True",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13393,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 563,
                                                                    end: 689,
                                                                    as_str(): "fn ran_out_of_names(b: Bool) -> u64 {\n    if let Bool::True = b {\n        return 101;\n    } else {\n        return 102;\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1300,
                                                                        end: 1316,
                                                                        as_str(): "ran_out_of_names",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1300,
                                                            end: 1328,
                                                            as_str(): "ran_out_of_names(Bool::True)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13394,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1297,
                                                        end: 1299,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1275,
                                            end: 1328,
                                            as_str(): "thats_all(Bool::True) == ran_out_of_names(Bool::True)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13395,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1268,
                                        end: 1274,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31786,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1268,
                            end: 1329,
                            as_str(): "assert(thats_all(Bool::True) == ran_out_of_names(Bool::True))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1268,
                    end: 1329,
                    as_str(): "assert(thats_all(Bool::True) == ran_out_of_names(Bool::True))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1335,
                                        end: 1341,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1365,
                                                            end: 1367,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1365,
                                                            end: 1367,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1365,
                                                        end: 1367,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1342,
                                                                        end: 1351,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 818,
                                                                            end: 819,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 86,
                                                                                    as_str(): "False",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 1,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1352,
                                                                                end: 1356,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1358,
                                                                                end: 1363,
                                                                                as_str(): "False",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1358,
                                                                                    end: 1363,
                                                                                    as_str(): "False",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1358,
                                                                            end: 1363,
                                                                            as_str(): "False",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13399,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 805,
                                                                    end: 908,
                                                                    as_str(): "fn thats_all(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1342,
                                                                        end: 1351,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1342,
                                                            end: 1364,
                                                            as_str(): "thats_all(Bool::False)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1368,
                                                                        end: 1384,
                                                                        as_str(): "ran_out_of_names",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 583,
                                                                            end: 584,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 86,
                                                                                    as_str(): "False",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 1,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1385,
                                                                                end: 1389,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1391,
                                                                                end: 1396,
                                                                                as_str(): "False",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1391,
                                                                                    end: 1396,
                                                                                    as_str(): "False",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1391,
                                                                            end: 1396,
                                                                            as_str(): "False",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13402,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 563,
                                                                    end: 689,
                                                                    as_str(): "fn ran_out_of_names(b: Bool) -> u64 {\n    if let Bool::True = b {\n        return 101;\n    } else {\n        return 102;\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1368,
                                                                        end: 1384,
                                                                        as_str(): "ran_out_of_names",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1368,
                                                            end: 1397,
                                                            as_str(): "ran_out_of_names(Bool::False)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13403,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1365,
                                                        end: 1367,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1342,
                                            end: 1397,
                                            as_str(): "thats_all(Bool::False) == ran_out_of_names(Bool::False)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13404,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1335,
                                        end: 1341,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31793,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1335,
                            end: 1398,
                            as_str(): "assert(thats_all(Bool::False) == ran_out_of_names(Bool::False))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1335,
                    end: 1398,
                    as_str(): "assert(thats_all(Bool::False) == ran_out_of_names(Bool::False))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1404,
                                        end: 1410,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1433,
                                                            end: 1435,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1433,
                                                            end: 1435,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1433,
                                                        end: 1435,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1411,
                                                                        end: 1420,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 818,
                                                                            end: 819,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 67,
                                                                                    end: 71,
                                                                                    as_str(): "True",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 0,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1421,
                                                                                end: 1425,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1427,
                                                                                end: 1431,
                                                                                as_str(): "True",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1427,
                                                                                    end: 1431,
                                                                                    as_str(): "True",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1427,
                                                                            end: 1431,
                                                                            as_str(): "True",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13408,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 805,
                                                                    end: 908,
                                                                    as_str(): "fn thats_all(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1411,
                                                                        end: 1420,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1411,
                                                            end: 1432,
                                                            as_str(): "thats_all(Bool::True)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1436,
                                                                        end: 1446,
                                                                        as_str(): "another_fn",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 705,
                                                                            end: 706,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 67,
                                                                                    end: 71,
                                                                                    as_str(): "True",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 0,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1447,
                                                                                end: 1451,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1453,
                                                                                end: 1457,
                                                                                as_str(): "True",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1453,
                                                                                    end: 1457,
                                                                                    as_str(): "True",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1453,
                                                                            end: 1457,
                                                                            as_str(): "True",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13411,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 691,
                                                                    end: 803,
                                                                    as_str(): "fn another_fn(b: Bool) -> u64 {\n    if let Bool::True = b {\n        return 101;\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1436,
                                                                        end: 1446,
                                                                        as_str(): "another_fn",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1436,
                                                            end: 1458,
                                                            as_str(): "another_fn(Bool::True)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13412,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1433,
                                                        end: 1435,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1411,
                                            end: 1458,
                                            as_str(): "thats_all(Bool::True) == another_fn(Bool::True)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13413,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1404,
                                        end: 1410,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31800,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1404,
                            end: 1459,
                            as_str(): "assert(thats_all(Bool::True) == another_fn(Bool::True))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1404,
                    end: 1459,
                    as_str(): "assert(thats_all(Bool::True) == another_fn(Bool::True))",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1465,
                                        end: 1471,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc1da400,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1495,
                                                            end: 1497,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1495,
                                                            end: 1497,
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
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1495,
                                                        end: 1497,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1472,
                                                                        end: 1481,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 818,
                                                                            end: 819,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 86,
                                                                                    as_str(): "False",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 1,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1482,
                                                                                end: 1486,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1488,
                                                                                end: 1493,
                                                                                as_str(): "False",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1488,
                                                                                    end: 1493,
                                                                                    as_str(): "False",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1488,
                                                                            end: 1493,
                                                                            as_str(): "False",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13417,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 805,
                                                                    end: 908,
                                                                    as_str(): "fn thats_all(b: Bool) -> u64 {\n    if let Bool::True = b {\n        101\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1472,
                                                                        end: 1481,
                                                                        as_str(): "thats_all",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1472,
                                                            end: 1494,
                                                            as_str(): "thats_all(Bool::False)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb532830,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1498,
                                                                        end: 1508,
                                                                        as_str(): "another_fn",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 705,
                                                                            end: 706,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: EnumInstantiation {
                                                                            enum_decl: TyEnumDeclaration {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                        ),
                                                                                        start: 56,
                                                                                        end: 60,
                                                                                        as_str(): "Bool",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 71,
                                                                                                as_str(): "True",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31630,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 75,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 67,
                                                                                            end: 75,
                                                                                            as_str(): "True: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 86,
                                                                                                as_str(): "False",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31633,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 90,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 90,
                                                                                            as_str(): "False: ()",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 93,
                                                                                    as_str(): "enum Bool {\n    True: (),\n    False: (),\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 86,
                                                                                    as_str(): "False",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 1,
                                                                            contents: None,
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1509,
                                                                                end: 1513,
                                                                                as_str(): "Bool",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1515,
                                                                                end: 1520,
                                                                                as_str(): "False",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 1515,
                                                                                    end: 1520,
                                                                                    as_str(): "False",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31668,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1515,
                                                                            end: 1520,
                                                                            as_str(): "False",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13420,
                                                                Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 691,
                                                                    end: 803,
                                                                    as_str(): "fn another_fn(b: Bool) -> u64 {\n    if let Bool::True = b {\n        return 101;\n    } else {\n        102\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1498,
                                                                        end: 1508,
                                                                        as_str(): "another_fn",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1498,
                                                            end: 1521,
                                                            as_str(): "another_fn(Bool::False)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13421,
                                                Span {
                                                    src (ptr): 0x00007fe0fb532830,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 1495,
                                                        end: 1497,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 1472,
                                            end: 1521,
                                            as_str(): "thats_all(Bool::False) == another_fn(Bool::False)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13422,
                                Span {
                                    src (ptr): 0x00007fe0dc1da400,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 1465,
                                        end: 1471,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31807,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1465,
                            end: 1522,
                            as_str(): "assert(thats_all(Bool::False) == another_fn(Bool::False))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1465,
                    end: 1522,
                    as_str(): "assert(thats_all(Bool::False) == another_fn(Bool::False))",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                2,
                            ),
                        ),
                        return_type: TypeId(
                            31808,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 1529,
                            end: 1530,
                            as_str(): "2",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0d2fa5ff0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                    ),
                    start: 1529,
                    end: 1530,
                    as_str(): "2",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 910,
        end: 1532,
        as_str(): "fn main() -> u64 {\n    assert(foo(true) == bar(true));\n    assert(foo(false) == bar(false));\n    assert(foo(true) == bell(true));\n    assert(foo(false) == bell(false));\n    assert(foo(true) == moo(true));\n    assert(foo(false) == moo(false));\n\n    assert(thats_all(Bool::True) == poo(Bool::True));\n    assert(thats_all(Bool::False) == poo(Bool::False));\n    assert(thats_all(Bool::True) == ran_out_of_names(Bool::True));\n    assert(thats_all(Bool::False) == ran_out_of_names(Bool::False));\n    assert(thats_all(Bool::True) == another_fn(Bool::True));\n    assert(thats_all(Bool::False) == another_fn(Bool::False));\n\n    2\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0d2fa5ff0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
        ),
        start: 923,
        end: 926,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

