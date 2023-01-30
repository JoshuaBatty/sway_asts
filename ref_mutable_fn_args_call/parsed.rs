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
                            src (ptr): 0x00007fe05beff8e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                            ),
                            start: 238,
                            end: 241,
                            as_str(): "foo",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
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
                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                    ),
                                                                    start: 264,
                                                                    end: 265,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 264,
                                                            end: 265,
                                                            as_str(): "x",
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
                                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                                    ),
                                                                                    start: 266,
                                                                                    end: 268,
                                                                                    as_str(): "+=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                                    ),
                                                                                    start: 266,
                                                                                    end: 268,
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
                                                                                src (ptr): 0x00007fe05beff8e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                                ),
                                                                                start: 266,
                                                                                end: 268,
                                                                                as_str(): "+=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                    ),
                                                                    start: 266,
                                                                    end: 268,
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
                                                                                src (ptr): 0x00007fe05beff8e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                                ),
                                                                                start: 264,
                                                                                end: 265,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05beff8e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                        ),
                                                                        start: 264,
                                                                        end: 265,
                                                                        as_str(): "x",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05beff8e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                        ),
                                                                        start: 269,
                                                                        end: 270,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05beff8e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                        ),
                                                        start: 264,
                                                        end: 270,
                                                        as_str(): "x += 1",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 264,
                                            end: 270,
                                            as_str(): "x += 1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 264,
                                    end: 270,
                                    as_str(): "x += 1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05beff8e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                            ),
                            start: 258,
                            end: 273,
                            as_str(): "{\n    x += 1;\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 250,
                                    end: 251,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: true,
                            is_mutable: true,
                            mutability_span: Span {
                                src (ptr): 0x00007fe05beff8e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                ),
                                start: 242,
                                end: 249,
                                as_str(): "ref mut",
                            },
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe05beff8e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                ),
                                start: 253,
                                end: 256,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe05beff8e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                        ),
                        start: 235,
                        end: 273,
                        as_str(): "fn foo(ref mut x: u64) {\n    x += 1;\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05beff8e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                        ),
                        start: 235,
                        end: 257,
                        as_str(): "fn foo(ref mut x: u64)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05beff8e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
            ),
            start: 235,
            end: 273,
            as_str(): "fn foo(ref mut x: u64) {\n    x += 1;\n}",
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
                            src (ptr): 0x00007fe05beff8e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                            ),
                            start: 278,
                            end: 282,
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
                                                    src (ptr): 0x00007fe05beff8e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                    ),
                                                    start: 306,
                                                    end: 307,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05beff8e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                    ),
                                                    start: 310,
                                                    end: 311,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 298,
                                    end: 312,
                                    as_str(): "let mut x = 1;",
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
                                                                src (ptr): 0x00007fe05beff8e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                ),
                                                                start: 317,
                                                                end: 320,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe05beff8e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                        ),
                                                        start: 317,
                                                        end: 320,
                                                        as_str(): "foo",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                    ),
                                                                    start: 321,
                                                                    end: 322,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 321,
                                                            end: 322,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 317,
                                            end: 323,
                                            as_str(): "foo(x)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 317,
                                    end: 323,
                                    as_str(): "foo(x)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05beff8e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                    ),
                                                    start: 329,
                                                    end: 330,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 329,
                                            end: 330,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 329,
                                    end: 330,
                                    as_str(): "x",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05beff8e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                            ),
                            start: 292,
                            end: 332,
                            as_str(): "{\n    let mut x = 1;\n    foo(x);\n    x\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe05beff8e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                        ),
                        start: 275,
                        end: 332,
                        as_str(): "fn main() -> u64 {\n    let mut x = 1;\n    foo(x);\n    x\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05beff8e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                        ),
                        start: 288,
                        end: 291,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05beff8e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
            ),
            start: 275,
            end: 332,
            as_str(): "fn main() -> u64 {\n    let mut x = 1;\n    foo(x);\n    x\n}",
        },
    },
]
