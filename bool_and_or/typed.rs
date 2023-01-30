TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb13ad4b060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
            ),
            start: 12,
            end: 17,
            as_str(): "abort",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: AsmExpression {
                            registers: [],
                            body: [],
                            returns: Some(
                                (
                                    AsmRegister {
                                        name: "one",
                                    },
                                    Span {
                                        src (ptr): 0x00007fb13ad4b060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                        ),
                                        start: 50,
                                        end: 53,
                                        as_str(): "one",
                                    },
                                ),
                            ),
                            whole_block_span: Span {
                                src (ptr): 0x00007fb13ad4b060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                ),
                                start: 34,
                                end: 77,
                                as_str(): "asm() {\n        one: bool // Failure.\n    }",
                            },
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb13ad4b060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                            ),
                            start: 34,
                            end: 77,
                            as_str(): "asm() {\n        one: bool // Failure.\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb13ad4b060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                    ),
                    start: 34,
                    end: 77,
                    as_str(): "asm() {\n        one: bool // Failure.\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb13ad4b060,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
        ),
        start: 9,
        end: 79,
        as_str(): "fn abort() -> bool {\n    asm() {\n        one: bool // Failure.\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb13ad4b060,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
        ),
        start: 23,
        end: 27,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb13ad4b060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
            ),
            start: 84,
            end: 88,
            as_str(): "main",
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
                                expression: LazyOperator {
                                    op: And,
                                    lhs: TyExpression {
                                        expression: LazyOperator {
                                            op: And,
                                            lhs: TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13ad4b060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                    ),
                                                    start: 108,
                                                    end: 112,
                                                    as_str(): "true",
                                                },
                                            },
                                            rhs: TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13ad4b060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                    ),
                                                    start: 116,
                                                    end: 121,
                                                    as_str(): "false",
                                                },
                                            },
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 108,
                                            end: 121,
                                            as_str(): "true && false",
                                        },
                                    },
                                    rhs: TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb13ad4b060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                        ),
                                                        start: 126,
                                                        end: 131,
                                                        as_str(): "abort",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: false,
                                            },
                                            contract_call_params: {},
                                            arguments: [],
                                            function_decl_id: DeclId(
                                                546,
                                                Span {
                                                    src (ptr): 0x00007fb13ad4b060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                    ),
                                                    start: 9,
                                                    end: 79,
                                                    as_str(): "fn abort() -> bool {\n    asm() {\n        one: bool // Failure.\n    }\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb13ad4b060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                        ),
                                                        start: 126,
                                                        end: 131,
                                                        as_str(): "abort",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 126,
                                            end: 133,
                                            as_str(): "abort()",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13ad4b060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                    ),
                                    start: 107,
                                    end: 133,
                                    as_str(): "(true && false) && abort()",
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
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13ad4b060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 165,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13ad4b060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                    ),
                                                    start: 164,
                                                    end: 165,
                                                    as_str(): "2",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13ad4b060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                    ),
                                    start: 134,
                                    end: 171,
                                    as_str(): "{\n        // Failure.\n        2\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: IfExp {
                                        condition: TyExpression {
                                            expression: LazyOperator {
                                                op: Or,
                                                lhs: TyExpression {
                                                    expression: LazyOperator {
                                                        op: Or,
                                                        lhs: TyExpression {
                                                            expression: Literal(
                                                                Boolean(
                                                                    false,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                71,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb13ad4b060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                ),
                                                                start: 181,
                                                                end: 186,
                                                                as_str(): "false",
                                                            },
                                                        },
                                                        rhs: TyExpression {
                                                            expression: Literal(
                                                                Boolean(
                                                                    true,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                71,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb13ad4b060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                ),
                                                                start: 190,
                                                                end: 194,
                                                                as_str(): "true",
                                                            },
                                                        },
                                                    },
                                                    return_type: TypeId(
                                                        71,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb13ad4b060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                        ),
                                                        start: 181,
                                                        end: 194,
                                                        as_str(): "false || true",
                                                    },
                                                },
                                                rhs: TyExpression {
                                                    expression: FunctionApplication {
                                                        call_path: CallPath {
                                                            prefixes: [],
                                                            suffix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                    ),
                                                                    start: 199,
                                                                    end: 204,
                                                                    as_str(): "abort",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_absolute: false,
                                                        },
                                                        contract_call_params: {},
                                                        arguments: [],
                                                        function_decl_id: DeclId(
                                                            548,
                                                            Span {
                                                                src (ptr): 0x00007fb13ad4b060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                ),
                                                                start: 9,
                                                                end: 79,
                                                                as_str(): "fn abort() -> bool {\n    asm() {\n        one: bool // Failure.\n    }\n}",
                                                            },
                                                        ),
                                                        self_state_idx: None,
                                                        selector: None,
                                                        type_binding: Some(
                                                            TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                    ),
                                                                    start: 199,
                                                                    end: 204,
                                                                    as_str(): "abort",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    return_type: TypeId(
                                                        71,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb13ad4b060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                        ),
                                                        start: 199,
                                                        end: 206,
                                                        as_str(): "abort()",
                                                    },
                                                },
                                            },
                                            return_type: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb13ad4b060,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                ),
                                                start: 180,
                                                end: 206,
                                                as_str(): "(false || true) || abort()",
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
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                        ),
                                                                        start: 237,
                                                                        end: 239,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb13ad4b060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                ),
                                                                start: 237,
                                                                end: 239,
                                                                as_str(): "42",
                                                            },
                                                        },
                                                    ],
                                                },
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb13ad4b060,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                ),
                                                start: 207,
                                                end: 245,
                                                as_str(): "{\n        // Success.\n        42\n    }",
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
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13ad4b060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                            ),
                                                                            start: 281,
                                                                            end: 282,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                    ),
                                                                    start: 281,
                                                                    end: 282,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13ad4b060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                    ),
                                                    start: 251,
                                                    end: 288,
                                                    as_str(): "{\n        // Failure.\n        3\n    }",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb13ad4b060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                        ),
                                        start: 177,
                                        end: 288,
                                        as_str(): "if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb13ad4b060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                            ),
                            start: 104,
                            end: 288,
                            as_str(): "if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb13ad4b060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                    ),
                    start: 104,
                    end: 288,
                    as_str(): "if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb13ad4b060,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
        ),
        start: 81,
        end: 290,
        as_str(): "fn main() -> u64 {\n    if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }\n}",
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
        src (ptr): 0x00007fb13ad4b060,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
        ),
        start: 94,
        end: 97,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

