[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0800b2770,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
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
                        src (ptr): 0x00007fe0800b2770,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                        ),
                        start: 9,
                        end: 20,
                        as_str(): "struct A {}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0800b2770,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
            ),
            start: 9,
            end: 20,
            as_str(): "struct A {}",
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
                                src (ptr): 0x00007fe0800b2770,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                ),
                                start: 27,
                                end: 28,
                                as_str(): "A",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0800b2770,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                        ),
                        start: 27,
                        end: 28,
                        as_str(): "A",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0800b2770,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                    ),
                                    start: 38,
                                    end: 45,
                                    as_str(): "generic",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 68,
                                                            end: 69,
                                                            as_str(): "x",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0800b2770,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                    ),
                                                    start: 68,
                                                    end: 69,
                                                    as_str(): "x",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 68,
                                            end: 69,
                                            as_str(): "x",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0800b2770,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                    ),
                                    start: 66,
                                    end: 71,
                                    as_str(): "{ x }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 49,
                                            end: 53,
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
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 49,
                                        end: 53,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 55,
                                            end: 56,
                                            as_str(): "x",
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
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0800b2770,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                ),
                                                start: 58,
                                                end: 59,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 58,
                                        end: 59,
                                        as_str(): "T",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0800b2770,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                ),
                                start: 35,
                                end: 71,
                                as_str(): "fn generic<T>(self, x: T) -> T { x }",
                            },
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 64,
                                        end: 65,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_parameters: [
                                T: TypeId(31628),
                            ],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0800b2770,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                ),
                                start: 64,
                                end: 65,
                                as_str(): "T",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0800b2770,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                        ),
                        start: 22,
                        end: 73,
                        as_str(): "impl A {\n    fn generic<T>(self, x: T) -> T { x }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0800b2770,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
            ),
            start: 22,
            end: 73,
            as_str(): "impl A {\n    fn generic<T>(self, x: T) -> T { x }\n}",
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
                            src (ptr): 0x00007fe0800b2770,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                            ),
                            start: 78,
                            end: 81,
                            as_str(): "foo",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                src (ptr): 0x00007fe0800b2770,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                ),
                                                                start: 103,
                                                                end: 110,
                                                                as_str(): "generic",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                71,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                71,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0800b2770,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                ),
                                                                start: 113,
                                                                end: 117,
                                                                as_str(): "bool",
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0800b2770,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                        ),
                                                        start: 103,
                                                        end: 117,
                                                        as_str(): "generic::<bool",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Struct(
                                                            StructExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0800b2770,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                                ),
                                                                                start: 98,
                                                                                end: 99,
                                                                                as_str(): "A",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0800b2770,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                        ),
                                                                        start: 98,
                                                                        end: 99,
                                                                        as_str(): "A",
                                                                    },
                                                                },
                                                                fields: [],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 102,
                                                            as_str(): "A {}",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 123,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 124,
                                            as_str(): "A {}.generic::<bool>(true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0800b2770,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                    ),
                                    start: 98,
                                    end: 124,
                                    as_str(): "A {}.generic::<bool>(true)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0800b2770,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                            ),
                            start: 92,
                            end: 126,
                            as_str(): "{\n    A {}.generic::<bool>(true)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0800b2770,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                        ),
                        start: 75,
                        end: 126,
                        as_str(): "fn foo() -> bool {\n    A {}.generic::<bool>(true)\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0800b2770,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                        ),
                        start: 87,
                        end: 91,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0800b2770,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
            ),
            start: 75,
            end: 126,
            as_str(): "fn foo() -> bool {\n    A {}.generic::<bool>(true)\n}",
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
                            src (ptr): 0x00007fe0800b2770,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                            ),
                            start: 131,
                            end: 135,
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
                                                name_override_opt: Some(
                                                    "_",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc01dd50,
                                                    path: None,
                                                    start: 0,
                                                    end: 0,
                                                    as_str(): "",
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
                                                                        src (ptr): 0x00007fe0800b2770,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                        ),
                                                                        start: 152,
                                                                        end: 155,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0800b2770,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                ),
                                                                start: 152,
                                                                end: 155,
                                                                as_str(): "foo",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0800b2770,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                    ),
                                                    start: 152,
                                                    end: 157,
                                                    as_str(): "foo()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0800b2770,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                    ),
                                    start: 144,
                                    end: 158,
                                    as_str(): "let _ = foo();",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0800b2770,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                            ),
                            start: 138,
                            end: 160,
                            as_str(): "{\n    let _ = foo();\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0800b2770,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                        ),
                        start: 128,
                        end: 160,
                        as_str(): "fn main() {\n    let _ = foo();\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0800b2770,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                        ),
                        start: 128,
                        end: 137,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0800b2770,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
            ),
            start: 128,
            end: 160,
            as_str(): "fn main() {\n    let _ = foo();\n}",
        },
    },
]
