[
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 15,
                            end: 25,
                            as_str(): "GLOBAL_NUM",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: FunctionApplication(
                            FunctionApplicationExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe07c9e90e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                ),
                                                start: 28,
                                                end: 36,
                                                as_str(): "a_number",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe07c9e90e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                        ),
                                        start: 28,
                                        end: 36,
                                        as_str(): "a_number",
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 37,
                                            end: 38,
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 41,
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 43,
                                            end: 44,
                                            as_str(): "3",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 28,
                            end: 45,
                            as_str(): "a_number(1, 2, 3)",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fe07c9e90e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                        ),
                        start: 9,
                        end: 46,
                        as_str(): "const GLOBAL_NUM = a_number(1, 2, 3);",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe07c9e90e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
            ),
            start: 9,
            end: 46,
            as_str(): "const GLOBAL_NUM = a_number(1, 2, 3);",
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
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 51,
                            end: 59,
                            as_str(): "a_number",
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 100,
                                            end: 102,
                                            as_str(): "42",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 100,
                                    end: 102,
                                    as_str(): "42",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 94,
                            end: 104,
                            as_str(): "{\n    42\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 60,
                                    end: 62,
                                    as_str(): "_a",
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
                                src (ptr): 0x00007fe07c9e90e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                ),
                                start: 64,
                                end: 67,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 69,
                                    end: 71,
                                    as_str(): "_b",
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
                                src (ptr): 0x00007fe07c9e90e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                ),
                                start: 73,
                                end: 76,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 78,
                                    end: 80,
                                    as_str(): "_c",
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
                                src (ptr): 0x00007fe07c9e90e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                ),
                                start: 82,
                                end: 85,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe07c9e90e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                        ),
                        start: 48,
                        end: 104,
                        as_str(): "fn a_number(_a: u64, _b: u64, _c: u64) -> u64 {\n    42\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe07c9e90e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                        ),
                        start: 90,
                        end: 93,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe07c9e90e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
            ),
            start: 48,
            end: 104,
            as_str(): "fn a_number(_a: u64, _b: u64, _c: u64) -> u64 {\n    42\n}",
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
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 109,
                            end: 113,
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
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 134,
                                                    as_str(): "a",
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
                                                                        src (ptr): 0x00007fe07c9e90e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                        ),
                                                                        start: 137,
                                                                        end: 145,
                                                                        as_str(): "a_number",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 137,
                                                                end: 145,
                                                                as_str(): "a_number",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        4,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                    ),
                                                                    start: 146,
                                                                    end: 147,
                                                                    as_str(): "4",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        5,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                    ),
                                                                    start: 149,
                                                                    end: 150,
                                                                    as_str(): "5",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        6,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                    ),
                                                                    start: 152,
                                                                    end: 153,
                                                                    as_str(): "6",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 137,
                                                    end: 154,
                                                    as_str(): "a_number(4, 5, 6)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 129,
                                    end: 155,
                                    as_str(): "let a = a_number(4, 5, 6);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 160,
                                                    end: 170,
                                                    as_str(): "GLOBAL_NUM",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 160,
                                            end: 170,
                                            as_str(): "GLOBAL_NUM",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 160,
                                    end: 170,
                                    as_str(): "GLOBAL_NUM",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 123,
                            end: 172,
                            as_str(): "{\n    let a = a_number(4, 5, 6);\n    GLOBAL_NUM\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe07c9e90e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                        ),
                        start: 106,
                        end: 172,
                        as_str(): "fn main() -> u64 {\n    let a = a_number(4, 5, 6);\n    GLOBAL_NUM\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe07c9e90e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                        ),
                        start: 119,
                        end: 122,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe07c9e90e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
            ),
            start: 106,
            end: 172,
            as_str(): "fn main() -> u64 {\n    let a = a_number(4, 5, 6);\n    GLOBAL_NUM\n}",
        },
    },
]
