[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0934e5bc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                            ),
                            start: 16,
                            end: 17,
                            as_str(): "A",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0934e5bc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                        ),
                        start: 9,
                        end: 21,
                        as_str(): "struct A { }",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0934e5bc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
            ),
            start: 9,
            end: 21,
            as_str(): "struct A { }",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0934e5bc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                ),
                                start: 28,
                                end: 29,
                                as_str(): "A",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0934e5bc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                        ),
                        start: 28,
                        end: 29,
                        as_str(): "A",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0934e5bc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                    ),
                                    start: 39,
                                    end: 40,
                                    as_str(): "f",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0934e5bc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                    ),
                                                    start: 64,
                                                    end: 65,
                                                    as_str(): "1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 64,
                                            end: 65,
                                            as_str(): "1",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0934e5bc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                    ),
                                    start: 54,
                                    end: 71,
                                    as_str(): "{\n        1\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 41,
                                            end: 45,
                                            as_str(): "self",
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
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 45,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0934e5bc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                ),
                                start: 36,
                                end: 71,
                                as_str(): "fn f(self) -> u64 {\n        1\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0934e5bc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                ),
                                start: 50,
                                end: 53,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0934e5bc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                        ),
                        start: 23,
                        end: 73,
                        as_str(): "impl A {\n    fn f(self) -> u64 {\n        1\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0934e5bc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
            ),
            start: 23,
            end: 73,
            as_str(): "impl A {\n    fn f(self) -> u64 {\n        1\n    }\n}",
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
                            src (ptr): 0x00007fe0934e5bc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                            ),
                            start: 78,
                            end: 82,
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
                                                    src (ptr): 0x00007fe0934e5bc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 103,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0934e5bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                        ),
                                                                        start: 106,
                                                                        end: 107,
                                                                        as_str(): "A",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0934e5bc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 107,
                                                                as_str(): "A",
                                                            },
                                                        },
                                                        fields: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0934e5bc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                    ),
                                                    start: 106,
                                                    end: 111,
                                                    as_str(): "A { }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0934e5bc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                    ),
                                    start: 98,
                                    end: 112,
                                    as_str(): "let a = A { };",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0934e5bc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                ),
                                                                start: 119,
                                                                end: 120,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0934e5bc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                        ),
                                                        start: 119,
                                                        end: 120,
                                                        as_str(): "f",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0934e5bc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 118,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0934e5bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                            ),
                                                            start: 117,
                                                            end: 118,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 122,
                                            as_str(): "a.f()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0934e5bc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                    ),
                                    start: 117,
                                    end: 122,
                                    as_str(): "a.f()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0934e5bc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                            ),
                            start: 92,
                            end: 124,
                            as_str(): "{\n    let a = A { };\n    a.f()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0934e5bc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                        ),
                        start: 75,
                        end: 124,
                        as_str(): "fn main() -> u64 {\n    let a = A { };\n    a.f()\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0934e5bc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                        ),
                        start: 88,
                        end: 91,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0934e5bc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
            ),
            start: 75,
            end: 124,
            as_str(): "fn main() -> u64 {\n    let a = A { };\n    a.f()\n}",
        },
    },
]
