[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
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
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
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
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
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
            src (ptr): 0x00007fb1355c1840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
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
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                            ),
                            start: 105,
                            end: 116,
                            as_str(): "blockheight",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Asm(
                                            AsmExpression {
                                                registers: [
                                                    AsmRegisterDeclaration {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 136,
                                                                end: 138,
                                                                as_str(): "r1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        initializer: None,
                                                    },
                                                ],
                                                body: [
                                                    AsmOp {
                                                        op_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 150,
                                                                end: 154,
                                                                as_str(): "bhei",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        op_args: [
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 155,
                                                                    end: 157,
                                                                    as_str(): "r1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 150,
                                                            end: 157,
                                                            as_str(): "bhei r1",
                                                        },
                                                        immediate: None,
                                                    },
                                                ],
                                                returns: Some(
                                                    (
                                                        AsmRegister {
                                                            name: "r1",
                                                        },
                                                        Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 169,
                                                            as_str(): "r1",
                                                        },
                                                    ),
                                                ),
                                                return_type: UnsignedInteger(
                                                    SixtyFour,
                                                ),
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 132,
                                                    end: 180,
                                                    as_str(): "asm(r1) {\n        bhei r1;\n        r1: u64\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 132,
                                            end: 180,
                                            as_str(): "asm(r1) {\n        bhei r1;\n        r1: u64\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 132,
                                    end: 180,
                                    as_str(): "asm(r1) {\n        bhei r1;\n        r1: u64\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                            ),
                            start: 126,
                            end: 182,
                            as_str(): "{\n    asm(r1) {\n        bhei r1;\n        r1: u64\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb1355c1840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                        ),
                        start: 102,
                        end: 182,
                        as_str(): "fn blockheight() -> u64 {\n    asm(r1) {\n        bhei r1;\n        r1: u64\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1355c1840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                        ),
                        start: 122,
                        end: 125,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1355c1840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
            ),
            start: 102,
            end: 182,
            as_str(): "fn blockheight() -> u64 {\n    asm(r1) {\n        bhei r1;\n        r1: u64\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                            ),
                            start: 191,
                            end: 200,
                            as_str(): "GasCounts",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 207,
                                    end: 217,
                                    as_str(): "global_gas",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb1355c1840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                ),
                                start: 207,
                                end: 222,
                                as_str(): "global_gas: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb1355c1840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                ),
                                start: 219,
                                end: 222,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 228,
                                    end: 239,
                                    as_str(): "context_gas",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb1355c1840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                ),
                                start: 228,
                                end: 244,
                                as_str(): "context_gas: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb1355c1840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                ),
                                start: 241,
                                end: 244,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb1355c1840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                        ),
                        start: 184,
                        end: 247,
                        as_str(): "struct GasCounts {\n    global_gas: u64,\n    context_gas: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1355c1840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
            ),
            start: 184,
            end: 247,
            as_str(): "struct GasCounts {\n    global_gas: u64,\n    context_gas: u64,\n}",
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
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                            ),
                            start: 252,
                            end: 259,
                            as_str(): "get_gas",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Struct(
                                            StructExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 281,
                                                                end: 290,
                                                                as_str(): "GasCounts",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1355c1840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                        ),
                                                        start: 281,
                                                        end: 290,
                                                        as_str(): "GasCounts",
                                                    },
                                                },
                                                fields: [
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 301,
                                                                end: 311,
                                                                as_str(): "global_gas",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Asm(
                                                                AsmExpression {
                                                                    registers: [],
                                                                    body: [],
                                                                    returns: Some(
                                                                        (
                                                                            AsmRegister {
                                                                                name: "ggas",
                                                                            },
                                                                            Span {
                                                                                src (ptr): 0x00007fb1355c1840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                ),
                                                                                start: 333,
                                                                                end: 337,
                                                                                as_str(): "ggas",
                                                                            },
                                                                        ),
                                                                    ),
                                                                    return_type: UnsignedInteger(
                                                                        SixtyFour,
                                                                    ),
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 313,
                                                                        end: 347,
                                                                        as_str(): "asm() {\n            ggas\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 313,
                                                                end: 347,
                                                                as_str(): "asm() {\n            ggas\n        }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 347,
                                                            as_str(): "global_gas: asm() {\n            ggas\n        }",
                                                        },
                                                    },
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 357,
                                                                end: 368,
                                                                as_str(): "context_gas",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Asm(
                                                                AsmExpression {
                                                                    registers: [],
                                                                    body: [],
                                                                    returns: Some(
                                                                        (
                                                                            AsmRegister {
                                                                                name: "cgas",
                                                                            },
                                                                            Span {
                                                                                src (ptr): 0x00007fb1355c1840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                ),
                                                                                start: 390,
                                                                                end: 394,
                                                                                as_str(): "cgas",
                                                                            },
                                                                        ),
                                                                    ),
                                                                    return_type: UnsignedInteger(
                                                                        SixtyFour,
                                                                    ),
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 370,
                                                                        end: 404,
                                                                        as_str(): "asm() {\n            cgas\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 370,
                                                                end: 404,
                                                                as_str(): "asm() {\n            cgas\n        }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 357,
                                                            end: 404,
                                                            as_str(): "context_gas: asm() {\n            cgas\n        }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 281,
                                            end: 410,
                                            as_str(): "GasCounts {\n        global_gas: asm() {\n            ggas\n        },\n        context_gas: asm() {\n            cgas\n        }\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 281,
                                    end: 410,
                                    as_str(): "GasCounts {\n        global_gas: asm() {\n            ggas\n        },\n        context_gas: asm() {\n            cgas\n        }\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                            ),
                            start: 275,
                            end: 412,
                            as_str(): "{\n    GasCounts {\n        global_gas: asm() {\n            ggas\n        },\n        context_gas: asm() {\n            cgas\n        }\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb1355c1840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                        ),
                        start: 249,
                        end: 412,
                        as_str(): "fn get_gas() -> GasCounts {\n    GasCounts {\n        global_gas: asm() {\n            ggas\n        },\n        context_gas: asm() {\n            cgas\n        }\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb1355c1840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                ),
                                start: 265,
                                end: 274,
                                as_str(): "GasCounts",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1355c1840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                        ),
                        start: 265,
                        end: 274,
                        as_str(): "GasCounts",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1355c1840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
            ),
            start: 249,
            end: 412,
            as_str(): "fn get_gas() -> GasCounts {\n    GasCounts {\n        global_gas: asm() {\n            ggas\n        },\n        context_gas: asm() {\n            cgas\n        }\n    }\n}",
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
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                            ),
                            start: 417,
                            end: 421,
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
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 441,
                                                    end: 453,
                                                    as_str(): "block_height",
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
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 456,
                                                                        end: 467,
                                                                        as_str(): "blockheight",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 456,
                                                                end: 467,
                                                                as_str(): "blockheight",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 456,
                                                    end: 469,
                                                    as_str(): "blockheight()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 437,
                                    end: 470,
                                    as_str(): "let block_height = blockheight();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 479,
                                                    end: 492,
                                                    as_str(): "remaining_gas",
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
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 495,
                                                                        end: 502,
                                                                        as_str(): "get_gas",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 495,
                                                                end: 502,
                                                                as_str(): "get_gas",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 495,
                                                    end: 504,
                                                    as_str(): "get_gas()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 475,
                                    end: 505,
                                    as_str(): "let remaining_gas = get_gas();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 565,
                                                    end: 569,
                                                    as_str(): "zero",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "zero",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 580,
                                                                    end: 584,
                                                                    as_str(): "zero",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 572,
                                                            end: 586,
                                                            as_str(): "asm() { zero }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 572,
                                                    end: 586,
                                                    as_str(): "asm() { zero }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 561,
                                    end: 587,
                                    as_str(): "let zero = asm() { zero };",
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 592,
                                                                end: 598,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1355c1840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                        ),
                                                        start: 592,
                                                        end: 598,
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
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 604,
                                                                                        end: 606,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 604,
                                                                                        end: 606,
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 604,
                                                                                    end: 606,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 604,
                                                                        end: 606,
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 599,
                                                                                    end: 603,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 599,
                                                                            end: 603,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 607,
                                                                            end: 608,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 599,
                                                            end: 608,
                                                            as_str(): "zero == 0",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 592,
                                            end: 609,
                                            as_str(): "assert(zero == 0)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 592,
                                    end: 609,
                                    as_str(): "assert(zero == 0)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 620,
                                                    end: 623,
                                                    as_str(): "one",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "one",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 634,
                                                                    end: 637,
                                                                    as_str(): "one",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 626,
                                                            end: 639,
                                                            as_str(): "asm() { one }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 626,
                                                    end: 639,
                                                    as_str(): "asm() { one }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 616,
                                    end: 640,
                                    as_str(): "let one = asm() { one };",
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 645,
                                                                end: 651,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1355c1840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                        ),
                                                        start: 645,
                                                        end: 651,
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
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 656,
                                                                                        end: 658,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 656,
                                                                                        end: 658,
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 656,
                                                                                    end: 658,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 656,
                                                                        end: 658,
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 652,
                                                                                    end: 655,
                                                                                    as_str(): "one",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 652,
                                                                            end: 655,
                                                                            as_str(): "one",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 659,
                                                                            end: 660,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 660,
                                                            as_str(): "one == 1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 645,
                                            end: 661,
                                            as_str(): "assert(one == 1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 645,
                                    end: 661,
                                    as_str(): "assert(one == 1)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 672,
                                                    end: 674,
                                                    as_str(): "of",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "of",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 685,
                                                                    end: 687,
                                                                    as_str(): "of",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 677,
                                                            end: 689,
                                                            as_str(): "asm() { of }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 677,
                                                    end: 689,
                                                    as_str(): "asm() { of }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 668,
                                    end: 690,
                                    as_str(): "let of = asm() { of };",
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 695,
                                                                end: 701,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1355c1840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                        ),
                                                        start: 695,
                                                        end: 701,
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
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 705,
                                                                                        end: 707,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 705,
                                                                                        end: 707,
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 705,
                                                                                    end: 707,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 705,
                                                                        end: 707,
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 702,
                                                                                    end: 704,
                                                                                    as_str(): "of",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 702,
                                                                            end: 704,
                                                                            as_str(): "of",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 708,
                                                                            end: 709,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 702,
                                                            end: 709,
                                                            as_str(): "of == 0",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 695,
                                            end: 710,
                                            as_str(): "assert(of == 0)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 695,
                                    end: 710,
                                    as_str(): "assert(of == 0)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 721,
                                                    end: 723,
                                                    as_str(): "pc",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "pc",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 734,
                                                                    end: 736,
                                                                    as_str(): "pc",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 726,
                                                            end: 738,
                                                            as_str(): "asm() { pc }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 726,
                                                    end: 738,
                                                    as_str(): "asm() { pc }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 717,
                                    end: 739,
                                    as_str(): "let pc = asm() { pc };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 749,
                                                    end: 752,
                                                    as_str(): "ssp",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "ssp",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 763,
                                                                    end: 766,
                                                                    as_str(): "ssp",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 755,
                                                            end: 768,
                                                            as_str(): "asm() { ssp }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 755,
                                                    end: 768,
                                                    as_str(): "asm() { ssp }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 745,
                                    end: 769,
                                    as_str(): "let ssp = asm() { ssp };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 779,
                                                    end: 781,
                                                    as_str(): "sp",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "sp",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 792,
                                                                    end: 794,
                                                                    as_str(): "sp",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 784,
                                                            end: 796,
                                                            as_str(): "asm() { sp }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 784,
                                                    end: 796,
                                                    as_str(): "asm() { sp }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 775,
                                    end: 797,
                                    as_str(): "let sp = asm() { sp };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 807,
                                                    end: 809,
                                                    as_str(): "fp",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "fp",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 820,
                                                                    end: 822,
                                                                    as_str(): "fp",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 812,
                                                            end: 824,
                                                            as_str(): "asm() { fp }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 812,
                                                    end: 824,
                                                    as_str(): "asm() { fp }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 803,
                                    end: 825,
                                    as_str(): "let fp = asm() { fp };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 835,
                                                    end: 837,
                                                    as_str(): "hp",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "hp",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 848,
                                                                    end: 850,
                                                                    as_str(): "hp",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 840,
                                                            end: 852,
                                                            as_str(): "asm() { hp }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 840,
                                                    end: 852,
                                                    as_str(): "asm() { hp }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 831,
                                    end: 853,
                                    as_str(): "let hp = asm() { hp };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 863,
                                                    end: 866,
                                                    as_str(): "err",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "err",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 877,
                                                                    end: 880,
                                                                    as_str(): "err",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 869,
                                                            end: 882,
                                                            as_str(): "asm() { err }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 869,
                                                    end: 882,
                                                    as_str(): "asm() { err }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 859,
                                    end: 883,
                                    as_str(): "let err = asm() { err };",
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 888,
                                                                end: 894,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1355c1840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                        ),
                                                        start: 888,
                                                        end: 894,
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
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 899,
                                                                                        end: 901,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 899,
                                                                                        end: 901,
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 899,
                                                                                    end: 901,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 899,
                                                                        end: 901,
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 895,
                                                                                    end: 898,
                                                                                    as_str(): "err",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 895,
                                                                            end: 898,
                                                                            as_str(): "err",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 902,
                                                                            end: 903,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 895,
                                                            end: 903,
                                                            as_str(): "err == 0",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 888,
                                            end: 904,
                                            as_str(): "assert(err == 0)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 888,
                                    end: 904,
                                    as_str(): "assert(err == 0)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 915,
                                                    end: 919,
                                                    as_str(): "ggas",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "ggas",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 930,
                                                                    end: 934,
                                                                    as_str(): "ggas",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 922,
                                                            end: 936,
                                                            as_str(): "asm() { ggas }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 922,
                                                    end: 936,
                                                    as_str(): "asm() { ggas }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 911,
                                    end: 937,
                                    as_str(): "let ggas = asm() { ggas };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 947,
                                                    end: 951,
                                                    as_str(): "cgas",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "cgas",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 962,
                                                                    end: 966,
                                                                    as_str(): "cgas",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 954,
                                                            end: 968,
                                                            as_str(): "asm() { cgas }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 954,
                                                    end: 968,
                                                    as_str(): "asm() { cgas }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 943,
                                    end: 969,
                                    as_str(): "let cgas = asm() { cgas };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 979,
                                                    end: 982,
                                                    as_str(): "bal",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "bal",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 993,
                                                                    end: 996,
                                                                    as_str(): "bal",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 985,
                                                            end: 998,
                                                            as_str(): "asm() { bal }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 985,
                                                    end: 998,
                                                    as_str(): "asm() { bal }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 975,
                                    end: 999,
                                    as_str(): "let bal = asm() { bal };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1009,
                                                    end: 1011,
                                                    as_str(): "is",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "is",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1022,
                                                                    end: 1024,
                                                                    as_str(): "is",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1014,
                                                            end: 1026,
                                                            as_str(): "asm() { is }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1014,
                                                    end: 1026,
                                                    as_str(): "asm() { is }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 1005,
                                    end: 1027,
                                    as_str(): "let is = asm() { is };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1037,
                                                    end: 1040,
                                                    as_str(): "ret",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "ret",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1051,
                                                                    end: 1054,
                                                                    as_str(): "ret",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1043,
                                                            end: 1056,
                                                            as_str(): "asm() { ret }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1043,
                                                    end: 1056,
                                                    as_str(): "asm() { ret }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 1033,
                                    end: 1057,
                                    as_str(): "let ret = asm() { ret };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1067,
                                                    end: 1071,
                                                    as_str(): "retl",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "retl",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1082,
                                                                    end: 1086,
                                                                    as_str(): "retl",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1074,
                                                            end: 1088,
                                                            as_str(): "asm() { retl }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1074,
                                                    end: 1088,
                                                    as_str(): "asm() { retl }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 1063,
                                    end: 1089,
                                    as_str(): "let retl = asm() { retl };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1099,
                                                    end: 1103,
                                                    as_str(): "flag",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [],
                                                        body: [],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "flag",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1114,
                                                                    end: 1118,
                                                                    as_str(): "flag",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1106,
                                                            end: 1120,
                                                            as_str(): "asm() { flag }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1106,
                                                    end: 1120,
                                                    as_str(): "asm() { flag }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 1095,
                                    end: 1121,
                                    as_str(): "let flag = asm() { flag };",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Literal(
                                                    U32(
                                                        6,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 1134,
                                                    end: 1138,
                                                    as_str(): "6u32",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 1127,
                                            end: 1138,
                                            as_str(): "return 6u32",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1355c1840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                    ),
                                    start: 1127,
                                    end: 1138,
                                    as_str(): "return 6u32",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                            ),
                            start: 431,
                            end: 1141,
                            as_str(): "{\n    let block_height = blockheight();\n    let remaining_gas = get_gas();\n\n    // Test the spelling of all special registers\n    let zero = asm() { zero };\n    assert(zero == 0);\n\n    let one = asm() { one };\n    assert(one == 1);\n\n    let of = asm() { of };\n    assert(of == 0);\n\n    let pc = asm() { pc };\n\n    let ssp = asm() { ssp };\n\n    let sp = asm() { sp };\n\n    let fp = asm() { fp };\n\n    let hp = asm() { hp };\n\n    let err = asm() { err };\n    assert(err == 0);\n\n    let ggas = asm() { ggas };\n\n    let cgas = asm() { cgas };\n\n    let bal = asm() { bal };\n\n    let is = asm() { is };\n\n    let ret = asm() { ret };\n\n    let retl = asm() { retl };\n\n    let flag = asm() { flag };\n\n    return 6u32;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb1355c1840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                        ),
                        start: 414,
                        end: 1141,
                        as_str(): "fn main() -> u32 {\n    let block_height = blockheight();\n    let remaining_gas = get_gas();\n\n    // Test the spelling of all special registers\n    let zero = asm() { zero };\n    assert(zero == 0);\n\n    let one = asm() { one };\n    assert(one == 1);\n\n    let of = asm() { of };\n    assert(of == 0);\n\n    let pc = asm() { pc };\n\n    let ssp = asm() { ssp };\n\n    let sp = asm() { sp };\n\n    let fp = asm() { fp };\n\n    let hp = asm() { hp };\n\n    let err = asm() { err };\n    assert(err == 0);\n\n    let ggas = asm() { ggas };\n\n    let cgas = asm() { cgas };\n\n    let bal = asm() { bal };\n\n    let is = asm() { is };\n\n    let ret = asm() { ret };\n\n    let retl = asm() { retl };\n\n    let flag = asm() { flag };\n\n    return 6u32;\n}",
                    },
                    return_type: UnsignedInteger(
                        ThirtyTwo,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1355c1840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                        ),
                        start: 427,
                        end: 430,
                        as_str(): "u32",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1355c1840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
            ),
            start: 414,
            end: 1141,
            as_str(): "fn main() -> u32 {\n    let block_height = blockheight();\n    let remaining_gas = get_gas();\n\n    // Test the spelling of all special registers\n    let zero = asm() { zero };\n    assert(zero == 0);\n\n    let one = asm() { one };\n    assert(one == 1);\n\n    let of = asm() { of };\n    assert(of == 0);\n\n    let pc = asm() { pc };\n\n    let ssp = asm() { ssp };\n\n    let sp = asm() { sp };\n\n    let fp = asm() { fp };\n\n    let hp = asm() { hp };\n\n    let err = asm() { err };\n    assert(err == 0);\n\n    let ggas = asm() { ggas };\n\n    let cgas = asm() { cgas };\n\n    let bal = asm() { bal };\n\n    let is = asm() { is };\n\n    let ret = asm() { ret };\n\n    let retl = asm() { retl };\n\n    let flag = asm() { flag };\n\n    return 6u32;\n}",
        },
    },
]
