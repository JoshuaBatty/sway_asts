[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
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
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 18,
                            end: 24,
                            as_str(): "revert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 26,
                            end: 32,
                            as_str(): "revert",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb1443204a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
            ),
            start: 9,
            end: 33,
            as_str(): "use std::revert::revert;",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 40,
                            end: 46,
                            as_str(): "Result",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(31628),
                        E: TypeId(31629),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 61,
                                    as_str(): "Ok",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                        ),
                                        start: 63,
                                        end: 64,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb1443204a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                ),
                                start: 63,
                                end: 64,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb1443204a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                ),
                                start: 59,
                                end: 64,
                                as_str(): "Ok: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 70,
                                    end: 73,
                                    as_str(): "Err",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                        ),
                                        start: 75,
                                        end: 76,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb1443204a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                ),
                                start: 75,
                                end: 76,
                                as_str(): "E",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb1443204a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                ),
                                start: 70,
                                end: 76,
                                as_str(): "Err: E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb1443204a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                        ),
                        start: 35,
                        end: 79,
                        as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1443204a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
            ),
            start: 35,
            end: 79,
            as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
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
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 84,
                            end: 95,
                            as_str(): "local_panic",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 112,
                                                        end: 120,
                                                        as_str(): "__revert",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Revert,
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 112,
                                                        end: 124,
                                                        as_str(): "__revert(42)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                42,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 121,
                                                            end: 123,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 112,
                                            end: 124,
                                            as_str(): "__revert(42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 112,
                                    end: 124,
                                    as_str(): "__revert(42)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 106,
                            end: 126,
                            as_str(): "{\n    __revert(42)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb1443204a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                        ),
                        start: 81,
                        end: 126,
                        as_str(): "fn local_panic<T>() -> T {\n    __revert(42)\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb1443204a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                ),
                                start: 104,
                                end: 105,
                                as_str(): "T",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        T: TypeId(31630),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1443204a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                        ),
                        start: 104,
                        end: 105,
                        as_str(): "T",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1443204a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
            ),
            start: 81,
            end: 126,
            as_str(): "fn local_panic<T>() -> T {\n    __revert(42)\n}",
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
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
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
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 265,
                                                    end: 266,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: If(
                                                    IfExpression {
                                                        condition: Expression {
                                                            kind: Literal(
                                                                Boolean(
                                                                    true,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 272,
                                                                end: 276,
                                                                as_str(): "true",
                                                            },
                                                        },
                                                        then: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            42,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 279,
                                                                                        end: 284,
                                                                                        as_str(): "42u64",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 279,
                                                                                end: 284,
                                                                                as_str(): "42u64",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 277,
                                                                        end: 286,
                                                                        as_str(): "{ 42u64 }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 277,
                                                                end: 286,
                                                                as_str(): "{ 42u64 }",
                                                            },
                                                        },
                                                        else: Some(
                                                            Expression {
                                                                kind: CodeBlock(
                                                                    CodeBlock {
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
                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                ),
                                                                                                                start: 294,
                                                                                                                end: 300,
                                                                                                                as_str(): "revert",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                        ),
                                                                                                        start: 294,
                                                                                                        end: 300,
                                                                                                        as_str(): "revert",
                                                                                                    },
                                                                                                },
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                0,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                            ),
                                                                                                            start: 301,
                                                                                                            end: 302,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 294,
                                                                                            end: 303,
                                                                                            as_str(): "revert(0)",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 294,
                                                                                    end: 303,
                                                                                    as_str(): "revert(0)",
                                                                                },
                                                                            },
                                                                        ],
                                                                        whole_block_span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 292,
                                                                            end: 305,
                                                                            as_str(): "{ revert(0) }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 292,
                                                                    end: 305,
                                                                    as_str(): "{ revert(0) }",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 269,
                                                    end: 305,
                                                    as_str(): "if true { 42u64 } else { revert(0) }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 261,
                                    end: 306,
                                    as_str(): "let x = if true { 42u64 } else { revert(0) };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 315,
                                                    end: 316,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 318,
                                                    end: 321,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 324,
                                                                        end: 335,
                                                                        as_str(): "local_panic",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 338,
                                                                        end: 341,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 324,
                                                                end: 342,
                                                                as_str(): "local_panic::<u64>",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 324,
                                                    end: 344,
                                                    as_str(): "local_panic::<u64>()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 311,
                                    end: 345,
                                    as_str(): "let x: u64 = local_panic::<u64>();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 354,
                                                    end: 355,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Match(
                                                    MatchExpression {
                                                        value: Expression {
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
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 382,
                                                                                            end: 388,
                                                                                            as_str(): "Result",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 382,
                                                                                        end: 388,
                                                                                        as_str(): "Result",
                                                                                    },
                                                                                },
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 390,
                                                                                        end: 392,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 395,
                                                                                    end: 398,
                                                                                    as_str(): "u64",
                                                                                },
                                                                            },
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 400,
                                                                                    end: 403,
                                                                                    as_str(): "u64",
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 382,
                                                                            end: 404,
                                                                            as_str(): "Result::Ok::<u64, u64>",
                                                                        },
                                                                    },
                                                                    args: [
                                                                        Expression {
                                                                            kind: Literal(
                                                                                Numeric(
                                                                                    5,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 405,
                                                                                end: 406,
                                                                                as_str(): "5",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 382,
                                                                end: 407,
                                                                as_str(): "Result::Ok::<u64, u64>(5)",
                                                            },
                                                        },
                                                        branches: [
                                                            MatchBranch {
                                                                scrutinee: EnumScrutinee {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 365,
                                                                                    end: 371,
                                                                                    as_str(): "Result",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 373,
                                                                                end: 375,
                                                                                as_str(): "Ok",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    value: Variable {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 376,
                                                                                end: 378,
                                                                                as_str(): "ok",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 376,
                                                                            end: 378,
                                                                            as_str(): "ok",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 365,
                                                                        end: 379,
                                                                        as_str(): "Result::Ok(ok)",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                        ),
                                                                                                        start: 418,
                                                                                                        end: 420,
                                                                                                        as_str(): "ok",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 418,
                                                                                                end: 420,
                                                                                                as_str(): "ok",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 418,
                                                                                        end: 420,
                                                                                        as_str(): "ok",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 408,
                                                                                end: 426,
                                                                                as_str(): "{\n        ok\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 408,
                                                                        end: 426,
                                                                        as_str(): "{\n        ok\n    }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 365,
                                                                    end: 426,
                                                                    as_str(): "Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    }",
                                                                },
                                                            },
                                                            MatchBranch {
                                                                scrutinee: CatchAll {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 432,
                                                                        end: 468,
                                                                        as_str(): "{\n        local_panic::<u64>()\n    }",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
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
                                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 442,
                                                                                                                    end: 453,
                                                                                                                    as_str(): "local_panic",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [
                                                                                                            TypeArgument {
                                                                                                                type_id: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                initial_type_id: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 456,
                                                                                                                    end: 459,
                                                                                                                    as_str(): "u64",
                                                                                                                },
                                                                                                            },
                                                                                                        ],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                            ),
                                                                                                            start: 442,
                                                                                                            end: 460,
                                                                                                            as_str(): "local_panic::<u64>",
                                                                                                        },
                                                                                                    },
                                                                                                    arguments: [],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 442,
                                                                                                end: 462,
                                                                                                as_str(): "local_panic::<u64>()",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 442,
                                                                                        end: 462,
                                                                                        as_str(): "local_panic::<u64>()",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 432,
                                                                                end: 468,
                                                                                as_str(): "{\n        local_panic::<u64>()\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 432,
                                                                        end: 468,
                                                                        as_str(): "{\n        local_panic::<u64>()\n    }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 432,
                                                                    end: 468,
                                                                    as_str(): "{\n        local_panic::<u64>()\n    }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 358,
                                                    end: 468,
                                                    as_str(): "if let Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    } else {\n        local_panic::<u64>()\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 350,
                                    end: 469,
                                    as_str(): "let x = if let Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    } else {\n        local_panic::<u64>()\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 478,
                                                    end: 479,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: If(
                                                    IfExpression {
                                                        condition: Expression {
                                                            kind: Literal(
                                                                Boolean(
                                                                    true,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 485,
                                                                end: 489,
                                                                as_str(): "true",
                                                            },
                                                        },
                                                        then: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
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
                                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 500,
                                                                                                                    end: 506,
                                                                                                                    as_str(): "Result",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                ),
                                                                                                                start: 500,
                                                                                                                end: 506,
                                                                                                                as_str(): "Result",
                                                                                                            },
                                                                                                        },
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                ),
                                                                                                                start: 508,
                                                                                                                end: 511,
                                                                                                                as_str(): "Err",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [
                                                                                                    TypeArgument {
                                                                                                        type_id: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        initial_type_id: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                            ),
                                                                                                            start: 514,
                                                                                                            end: 517,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                    },
                                                                                                    TypeArgument {
                                                                                                        type_id: TypeId(
                                                                                                            33,
                                                                                                        ),
                                                                                                        initial_type_id: TypeId(
                                                                                                            33,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                            ),
                                                                                                            start: 519,
                                                                                                            end: 522,
                                                                                                            as_str(): "u32",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                    ),
                                                                                                    start: 500,
                                                                                                    end: 523,
                                                                                                    as_str(): "Result::Err::<u64, u32>",
                                                                                                },
                                                                                            },
                                                                                            args: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            12,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                        ),
                                                                                                        start: 524,
                                                                                                        end: 526,
                                                                                                        as_str(): "12",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 500,
                                                                                        end: 527,
                                                                                        as_str(): "Result::Err::<u64, u32>(12)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 500,
                                                                                end: 527,
                                                                                as_str(): "Result::Err::<u64, u32>(12)",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 490,
                                                                        end: 533,
                                                                        as_str(): "{\n        Result::Err::<u64, u32>(12)\n    }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 490,
                                                                end: 533,
                                                                as_str(): "{\n        Result::Err::<u64, u32>(12)\n    }",
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
                                                                                        kind: Return(
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        10,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                    ),
                                                                                                    start: 556,
                                                                                                    end: 558,
                                                                                                    as_str(): "10",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 549,
                                                                                            end: 558,
                                                                                            as_str(): "return 10",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 549,
                                                                                    end: 558,
                                                                                    as_str(): "return 10",
                                                                                },
                                                                            },
                                                                        ],
                                                                        whole_block_span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 539,
                                                                            end: 565,
                                                                            as_str(): "{\n        return 10;\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 539,
                                                                    end: 565,
                                                                    as_str(): "{\n        return 10;\n    }",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 482,
                                                    end: 565,
                                                    as_str(): "if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 474,
                                    end: 566,
                                    as_str(): "let x = if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    };",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        42,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 578,
                                                    end: 580,
                                                    as_str(): "42",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 571,
                                            end: 580,
                                            as_str(): "return 42",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 571,
                                    end: 580,
                                    as_str(): "return 42",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 145,
                            end: 583,
                            as_str(): "{\n    // all of these should be okay, since\n    // the branches that would have type errors abort control flow.\n    let x = if true { 42u64 } else { revert(0) };\n    let x: u64 = local_panic::<u64>();\n    let x = if let Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    } else {\n        local_panic::<u64>()\n    };\n    let x = if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    };\n    return 42;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb1443204a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                        ),
                        start: 128,
                        end: 583,
                        as_str(): "fn main() -> u64 {\n    // all of these should be okay, since\n    // the branches that would have type errors abort control flow.\n    let x = if true { 42u64 } else { revert(0) };\n    let x: u64 = local_panic::<u64>();\n    let x = if let Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    } else {\n        local_panic::<u64>()\n    };\n    let x = if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    };\n    return 42;\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1443204a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                        ),
                        start: 141,
                        end: 144,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1443204a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
            ),
            start: 128,
            end: 583,
            as_str(): "fn main() -> u64 {\n    // all of these should be okay, since\n    // the branches that would have type errors abort control flow.\n    let x = if true { 42u64 } else { revert(0) };\n    let x: u64 = local_panic::<u64>();\n    let x = if let Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    } else {\n        local_panic::<u64>()\n    };\n    let x = if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    };\n    return 42;\n}",
        },
    },
]
