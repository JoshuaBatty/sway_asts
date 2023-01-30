[
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12d4a1aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                            ),
                            start: 15,
                            end: 25,
                            as_str(): "GLOBAL_VAL",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb12d4a1aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                            ),
                            start: 27,
                            end: 30,
                            as_str(): "u64",
                        },
                    ),
                    value: Expression {
                        kind: Literal(
                            Numeric(
                                99,
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12d4a1aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                            ),
                            start: 33,
                            end: 35,
                            as_str(): "99",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb12d4a1aa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                        ),
                        start: 9,
                        end: 36,
                        as_str(): "const GLOBAL_VAL: u64 = 99;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12d4a1aa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
            ),
            start: 9,
            end: 36,
            as_str(): "const GLOBAL_VAL: u64 = 99;",
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
                            src (ptr): 0x00007fb12d4a1aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                            ),
                            start: 41,
                            end: 45,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    ConstantDeclaration(
                                        ConstantDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                    ),
                                                    start: 67,
                                                    end: 76,
                                                    as_str(): "LOCAL_VAL",
                                                },
                                                is_raw_ident: false,
                                            },
                                            attributes: {},
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            value: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                    ),
                                                    start: 79,
                                                    end: 80,
                                                    as_str(): "1",
                                                },
                                            },
                                            visibility: Private,
                                            is_configurable: false,
                                            span: Span {
                                                src (ptr): 0x00007fb12d4a1aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                ),
                                                start: 61,
                                                end: 81,
                                                as_str(): "const LOCAL_VAL = 1;",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12d4a1aa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                    ),
                                    start: 61,
                                    end: 81,
                                    as_str(): "const LOCAL_VAL = 1;",
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
                                                                        src (ptr): 0x00007fb12d4a1aa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                        ),
                                                                        start: 97,
                                                                        end: 98,
                                                                        as_str(): "+",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "ops",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12d4a1aa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                        ),
                                                                        start: 97,
                                                                        end: 98,
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
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 97,
                                                                    end: 98,
                                                                    as_str(): "+",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_absolute: true,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12d4a1aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                        ),
                                                        start: 97,
                                                        end: 98,
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
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 86,
                                                                    end: 96,
                                                                    as_str(): "GLOBAL_VAL",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12d4a1aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                            ),
                                                            start: 86,
                                                            end: 96,
                                                            as_str(): "GLOBAL_VAL",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                                    ),
                                                                    start: 99,
                                                                    end: 108,
                                                                    as_str(): "LOCAL_VAL",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12d4a1aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                            ),
                                                            start: 99,
                                                            end: 108,
                                                            as_str(): "LOCAL_VAL",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12d4a1aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                            ),
                                            start: 86,
                                            end: 108,
                                            as_str(): "GLOBAL_VAL + LOCAL_VAL",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12d4a1aa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                    ),
                                    start: 86,
                                    end: 108,
                                    as_str(): "GLOBAL_VAL + LOCAL_VAL",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12d4a1aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                            ),
                            start: 55,
                            end: 110,
                            as_str(): "{\n    const LOCAL_VAL = 1;\n    GLOBAL_VAL + LOCAL_VAL\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12d4a1aa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                        ),
                        start: 38,
                        end: 110,
                        as_str(): "fn main() -> u64 {\n    const LOCAL_VAL = 1;\n    GLOBAL_VAL + LOCAL_VAL\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12d4a1aa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                        ),
                        start: 51,
                        end: 54,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12d4a1aa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
            ),
            start: 38,
            end: 110,
            as_str(): "fn main() -> u64 {\n    const LOCAL_VAL = 1;\n    GLOBAL_VAL + LOCAL_VAL\n}",
        },
    },
]
