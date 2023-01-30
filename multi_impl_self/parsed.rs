[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08f6888f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                            ),
                            start: 16,
                            end: 24,
                            as_str(): "MyStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 9,
                        end: 28,
                        as_str(): "struct MyStruct {\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 9,
            end: 28,
            as_str(): "struct MyStruct {\n}",
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
                                src (ptr): 0x00007fe08f6888f0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                ),
                                start: 35,
                                end: 43,
                                as_str(): "MyStruct",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 35,
                        end: 43,
                        as_str(): "MyStruct",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08f6888f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                    ),
                                    start: 57,
                                    end: 63,
                                    as_str(): "my_fun",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Public,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6888f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                        ),
                                                                        start: 83,
                                                                        end: 86,
                                                                        as_str(): "fun",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f6888f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                ),
                                                                start: 83,
                                                                end: 86,
                                                                as_str(): "fun",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f6888f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                    ),
                                                    start: 83,
                                                    end: 88,
                                                    as_str(): "fun()",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 83,
                                            end: 88,
                                            as_str(): "fun()",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe08f6888f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                    ),
                                    start: 73,
                                    end: 94,
                                    as_str(): "{\n        fun()\n    }",
                                },
                            },
                            parameters: [],
                            span: Span {
                                src (ptr): 0x00007fe08f6888f0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                ),
                                start: 50,
                                end: 94,
                                as_str(): "pub fn my_fun() -> u64 {\n        fun()\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe08f6888f0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                ),
                                start: 69,
                                end: 72,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 30,
                        end: 96,
                        as_str(): "impl MyStruct {\n    pub fn my_fun() -> u64 {\n        fun()\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 30,
            end: 96,
            as_str(): "impl MyStruct {\n    pub fn my_fun() -> u64 {\n        fun()\n    }\n}",
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
                                src (ptr): 0x00007fe08f6888f0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                ),
                                start: 103,
                                end: 111,
                                as_str(): "MyStruct",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 103,
                        end: 111,
                        as_str(): "MyStruct",
                    },
                    functions: [],
                    block_span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 98,
                        end: 115,
                        as_str(): "impl MyStruct {\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 98,
            end: 115,
            as_str(): "impl MyStruct {\n}",
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
                            src (ptr): 0x00007fe08f6888f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                            ),
                            start: 120,
                            end: 123,
                            as_str(): "fun",
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
                                                42,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 139,
                                            end: 141,
                                            as_str(): "42",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f6888f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                    ),
                                    start: 139,
                                    end: 141,
                                    as_str(): "42",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe08f6888f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                            ),
                            start: 133,
                            end: 143,
                            as_str(): "{\n    42\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 117,
                        end: 143,
                        as_str(): "fn fun() -> u64 {\n    42\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 129,
                        end: 132,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 117,
            end: 143,
            as_str(): "fn fun() -> u64 {\n    42\n}",
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
                            src (ptr): 0x00007fe08f6888f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                            ),
                            start: 148,
                            end: 152,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
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
                                                                        src (ptr): 0x00007fe08f6888f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                        ),
                                                                        start: 168,
                                                                        end: 176,
                                                                        as_str(): "MyStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f6888f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 168,
                                                                    end: 176,
                                                                    as_str(): "MyStruct",
                                                                },
                                                            },
                                                            suffix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f6888f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 178,
                                                                    end: 184,
                                                                    as_str(): "my_fun",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f6888f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                        ),
                                                        start: 168,
                                                        end: 184,
                                                        as_str(): "MyStruct::my_fun",
                                                    },
                                                },
                                                args: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 168,
                                            end: 186,
                                            as_str(): "MyStruct::my_fun()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f6888f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                    ),
                                    start: 168,
                                    end: 186,
                                    as_str(): "MyStruct::my_fun()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe08f6888f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                            ),
                            start: 162,
                            end: 188,
                            as_str(): "{\n    MyStruct::my_fun()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 145,
                        end: 188,
                        as_str(): "fn main() -> u64 {\n    MyStruct::my_fun()\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                        ),
                        start: 158,
                        end: 161,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 145,
            end: 188,
            as_str(): "fn main() -> u64 {\n    MyStruct::my_fun()\n}",
        },
    },
]
