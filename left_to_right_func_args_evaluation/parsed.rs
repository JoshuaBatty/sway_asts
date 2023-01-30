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
                            src (ptr): 0x00007fe0bd11adc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                            ),
                            start: 93,
                            end: 97,
                            as_str(): "func",
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
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 144,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 143,
                                            end: 144,
                                            as_str(): "d",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 143,
                                    end: 144,
                                    as_str(): "d",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0bd11adc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                            ),
                            start: 137,
                            end: 146,
                            as_str(): "{\n    d\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 98,
                                    end: 99,
                                    as_str(): "a",
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
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 101,
                                end: 104,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 106,
                                    end: 107,
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 109,
                                end: 112,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 115,
                                    as_str(): "c",
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
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 117,
                                end: 120,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 122,
                                    end: 123,
                                    as_str(): "d",
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
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 125,
                                end: 128,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0bd11adc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                        ),
                        start: 90,
                        end: 146,
                        as_str(): "fn func(a: u64, b: u64, c: u64, d: u64) -> u64 {\n    d\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0bd11adc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                        ),
                        start: 133,
                        end: 136,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0bd11adc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
            ),
            start: 90,
            end: 146,
            as_str(): "fn func(a: u64, b: u64, c: u64, d: u64) -> u64 {\n    d\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bd11adc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                            ),
                            start: 155,
                            end: 163,
                            as_str(): "MyStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 170,
                                    end: 171,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 170,
                                end: 176,
                                as_str(): "a: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 173,
                                end: 176,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 182,
                                    end: 183,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 182,
                                end: 188,
                                as_str(): "b: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 185,
                                end: 188,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 194,
                                    end: 195,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 194,
                                end: 200,
                                as_str(): "c: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 197,
                                end: 200,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 206,
                                    end: 207,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 206,
                                end: 212,
                                as_str(): "d: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0bd11adc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                ),
                                start: 209,
                                end: 212,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0bd11adc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                        ),
                        start: 148,
                        end: 215,
                        as_str(): "struct MyStruct {\n    a: u64,\n    b: u64,\n    c: u64,\n    d: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0bd11adc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
            ),
            start: 148,
            end: 215,
            as_str(): "struct MyStruct {\n    a: u64,\n    b: u64,\n    c: u64,\n    d: u64,\n}",
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
                            src (ptr): 0x00007fe0bd11adc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                            ),
                            start: 220,
                            end: 224,
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
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 249,
                                                    end: 250,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 252,
                                                    end: 255,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 258,
                                                    end: 259,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 241,
                                    end: 260,
                                    as_str(): "let mut x: u64 = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 307,
                                                    end: 315,
                                                    as_str(): "func_res",
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
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 326,
                                                                        end: 330,
                                                                        as_str(): "func",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 326,
                                                                end: 330,
                                                                as_str(): "func",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: CodeBlock(
                                                                    CodeBlock {
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
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 362,
                                                                                                                    end: 363,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 362,
                                                                                                            end: 363,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                rhs: Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            1,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 366,
                                                                                                        end: 367,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 362,
                                                                                            end: 367,
                                                                                            as_str(): "x = 1",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 362,
                                                                                    end: 367,
                                                                                    as_str(): "x = 1",
                                                                                },
                                                                            },
                                                                            AstNode {
                                                                                content: ImplicitReturnExpression(
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                0,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 385,
                                                                                            end: 386,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 385,
                                                                                    end: 386,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ],
                                                                        whole_block_span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 344,
                                                                            end: 400,
                                                                            as_str(): "{\n                x = 1;\n                0\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 344,
                                                                    end: 400,
                                                                    as_str(): "{\n                x = 1;\n                0\n            }",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: CodeBlock(
                                                                    CodeBlock {
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
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 432,
                                                                                                                    end: 433,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 432,
                                                                                                            end: 433,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                rhs: Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            2,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 436,
                                                                                                        end: 437,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 432,
                                                                                            end: 437,
                                                                                            as_str(): "x = 2",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 432,
                                                                                    end: 437,
                                                                                    as_str(): "x = 2",
                                                                                },
                                                                            },
                                                                            AstNode {
                                                                                content: ImplicitReturnExpression(
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                0,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 455,
                                                                                            end: 456,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 455,
                                                                                    end: 456,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ],
                                                                        whole_block_span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 414,
                                                                            end: 470,
                                                                            as_str(): "{\n                x = 2;\n                0\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 414,
                                                                    end: 470,
                                                                    as_str(): "{\n                x = 2;\n                0\n            }",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: CodeBlock(
                                                                    CodeBlock {
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
                                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 502,
                                                                                                                    end: 503,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 502,
                                                                                                            end: 503,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                rhs: Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            3,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 506,
                                                                                                        end: 507,
                                                                                                        as_str(): "3",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 502,
                                                                                            end: 507,
                                                                                            as_str(): "x = 3",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 502,
                                                                                    end: 507,
                                                                                    as_str(): "x = 3",
                                                                                },
                                                                            },
                                                                            AstNode {
                                                                                content: ImplicitReturnExpression(
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                0,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 525,
                                                                                            end: 526,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                    ),
                                                                                    start: 525,
                                                                                    end: 526,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ],
                                                                        whole_block_span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 484,
                                                                            end: 540,
                                                                            as_str(): "{\n                x = 3;\n                0\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 484,
                                                                    end: 540,
                                                                    as_str(): "{\n                x = 3;\n                0\n            }",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 554,
                                                                            end: 555,
                                                                            as_str(): "x",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 554,
                                                                    end: 555,
                                                                    as_str(): "x",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 326,
                                                    end: 565,
                                                    as_str(): "func(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                0\n            },\n            {\n                x = 3;\n                0\n            },\n            x\n        )",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 303,
                                    end: 566,
                                    as_str(): "let func_res =\n        func(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                0\n            },\n            {\n                x = 3;\n                0\n            },\n            x\n        );",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 596,
                                                                    end: 597,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 596,
                                                            end: 597,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            0,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bd11adc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                        ),
                                                        start: 600,
                                                        end: 601,
                                                        as_str(): "0",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 596,
                                            end: 601,
                                            as_str(): "x = 0",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 596,
                                    end: 601,
                                    as_str(): "x = 0",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 611,
                                                    end: 620,
                                                    as_str(): "tuple_res",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
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
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 663,
                                                                                                                end: 664,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 663,
                                                                                                        end: 664,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            rhs: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 667,
                                                                                                    end: 668,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 663,
                                                                                        end: 668,
                                                                                        as_str(): "x = 1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 663,
                                                                                end: 668,
                                                                                as_str(): "x = 1",
                                                                            },
                                                                        },
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 686,
                                                                                        end: 687,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 686,
                                                                                end: 687,
                                                                                as_str(): "0",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 645,
                                                                        end: 701,
                                                                        as_str(): "{\n                x = 1;\n                0\n            }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 645,
                                                                end: 701,
                                                                as_str(): "{\n                x = 1;\n                0\n            }",
                                                            },
                                                        },
                                                        Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
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
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 733,
                                                                                                                end: 734,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 733,
                                                                                                        end: 734,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            rhs: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        2,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 737,
                                                                                                    end: 738,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 733,
                                                                                        end: 738,
                                                                                        as_str(): "x = 2",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 733,
                                                                                end: 738,
                                                                                as_str(): "x = 2",
                                                                            },
                                                                        },
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            1,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 756,
                                                                                        end: 757,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 756,
                                                                                end: 757,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 715,
                                                                        end: 771,
                                                                        as_str(): "{\n                x = 2;\n                1\n            }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 715,
                                                                end: 771,
                                                                as_str(): "{\n                x = 2;\n                1\n            }",
                                                            },
                                                        },
                                                        Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
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
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 803,
                                                                                                                end: 804,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 803,
                                                                                                        end: 804,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            rhs: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        3,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 807,
                                                                                                    end: 808,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 803,
                                                                                        end: 808,
                                                                                        as_str(): "x = 3",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 803,
                                                                                end: 808,
                                                                                as_str(): "x = 3",
                                                                            },
                                                                        },
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            2,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 826,
                                                                                        end: 827,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 826,
                                                                                end: 827,
                                                                                as_str(): "2",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 785,
                                                                        end: 841,
                                                                        as_str(): "{\n                x = 3;\n                2\n            }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 785,
                                                                end: 841,
                                                                as_str(): "{\n                x = 3;\n                2\n            }",
                                                            },
                                                        },
                                                        Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 855,
                                                                        end: 856,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 855,
                                                                end: 856,
                                                                as_str(): "x",
                                                            },
                                                        },
                                                    ],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 631,
                                                    end: 866,
                                                    as_str(): "(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                1\n            },\n            {\n                x = 3;\n                2\n            },\n            x\n        )",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 607,
                                    end: 867,
                                    as_str(): "let tuple_res =\n        (\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                1\n            },\n            {\n                x = 3;\n                2\n            },\n            x\n        );",
                                },
                            },
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
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 898,
                                                                    end: 899,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd11adc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                            ),
                                                            start: 898,
                                                            end: 899,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            0,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bd11adc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                        ),
                                                        start: 902,
                                                        end: 903,
                                                        as_str(): "0",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 898,
                                            end: 903,
                                            as_str(): "x = 0",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 898,
                                    end: 903,
                                    as_str(): "x = 0",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 913,
                                                    end: 923,
                                                    as_str(): "struct_res",
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
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 934,
                                                                        end: 942,
                                                                        as_str(): "MyStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 934,
                                                                end: 942,
                                                                as_str(): "MyStruct",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 957,
                                                                        end: 958,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
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
                                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 978,
                                                                                                                        end: 979,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 978,
                                                                                                                end: 979,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    rhs: Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                1,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 982,
                                                                                                            end: 983,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 978,
                                                                                                end: 983,
                                                                                                as_str(): "x = 1",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 978,
                                                                                        end: 983,
                                                                                        as_str(): "x = 1",
                                                                                    },
                                                                                },
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    0,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1001,
                                                                                                end: 1002,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1001,
                                                                                        end: 1002,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 960,
                                                                                end: 1016,
                                                                                as_str(): "{\n                x = 1;\n                0\n            }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 960,
                                                                        end: 1016,
                                                                        as_str(): "{\n                x = 1;\n                0\n            }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 957,
                                                                    end: 1016,
                                                                    as_str(): "a: {\n                x = 1;\n                0\n            }",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 1030,
                                                                        end: 1031,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
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
                                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1051,
                                                                                                                        end: 1052,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1051,
                                                                                                                end: 1052,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    rhs: Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                2,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1055,
                                                                                                            end: 1056,
                                                                                                            as_str(): "2",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1051,
                                                                                                end: 1056,
                                                                                                as_str(): "x = 2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1051,
                                                                                        end: 1056,
                                                                                        as_str(): "x = 2",
                                                                                    },
                                                                                },
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    1,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1074,
                                                                                                end: 1075,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1074,
                                                                                        end: 1075,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1033,
                                                                                end: 1089,
                                                                                as_str(): "{\n                x = 2;\n                1\n            }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 1033,
                                                                        end: 1089,
                                                                        as_str(): "{\n                x = 2;\n                1\n            }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 1030,
                                                                    end: 1089,
                                                                    as_str(): "b: {\n                x = 2;\n                1\n            }",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 1103,
                                                                        end: 1104,
                                                                        as_str(): "c",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
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
                                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1124,
                                                                                                                        end: 1125,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1124,
                                                                                                                end: 1125,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    rhs: Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                3,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1128,
                                                                                                            end: 1129,
                                                                                                            as_str(): "3",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1124,
                                                                                                end: 1129,
                                                                                                as_str(): "x = 3",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1124,
                                                                                        end: 1129,
                                                                                        as_str(): "x = 3",
                                                                                    },
                                                                                },
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    2,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                ),
                                                                                                start: 1147,
                                                                                                end: 1148,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1147,
                                                                                        end: 1148,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1106,
                                                                                end: 1162,
                                                                                as_str(): "{\n                x = 3;\n                2\n            }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 1106,
                                                                        end: 1162,
                                                                        as_str(): "{\n                x = 3;\n                2\n            }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 1103,
                                                                    end: 1162,
                                                                    as_str(): "c: {\n                x = 3;\n                2\n            }",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 1176,
                                                                        end: 1177,
                                                                        as_str(): "d",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1179,
                                                                                end: 1180,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                        ),
                                                                        start: 1179,
                                                                        end: 1180,
                                                                        as_str(): "x",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                    ),
                                                                    start: 1176,
                                                                    end: 1180,
                                                                    as_str(): "d: x",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 934,
                                                    end: 1190,
                                                    as_str(): "MyStruct {\n            a: {\n                x = 1;\n                0\n            },\n            b: {\n                x = 2;\n                1\n            },\n            c: {\n                x = 3;\n                2\n            },\n            d: x\n        }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 909,
                                    end: 1191,
                                    as_str(): "let struct_res =\n        MyStruct {\n            a: {\n                x = 1;\n                0\n            },\n            b: {\n                x = 2;\n                1\n            },\n            c: {\n                x = 3;\n                2\n            },\n            d: x\n        };",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: LazyOperator(
                                                    LazyOperatorExpression {
                                                        op: And,
                                                        lhs: Expression {
                                                            kind: LazyOperator(
                                                                LazyOperatorExpression {
                                                                    op: And,
                                                                    lhs: Expression {
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
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1214,
                                                                                                        end: 1216,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1214,
                                                                                                        end: 1216,
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
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1214,
                                                                                                    end: 1216,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1214,
                                                                                        end: 1216,
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
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1205,
                                                                                                    end: 1213,
                                                                                                    as_str(): "func_res",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1205,
                                                                                            end: 1213,
                                                                                            as_str(): "func_res",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                3,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1217,
                                                                                            end: 1218,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 1205,
                                                                            end: 1218,
                                                                            as_str(): "func_res == 3",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1236,
                                                                                                        end: 1238,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1236,
                                                                                                        end: 1238,
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
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1236,
                                                                                                    end: 1238,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1236,
                                                                                        end: 1238,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: TupleIndex(
                                                                                            TupleIndexExpression {
                                                                                                prefix: Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1224,
                                                                                                                end: 1233,
                                                                                                                as_str(): "tuple_res",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1224,
                                                                                                        end: 1233,
                                                                                                        as_str(): "tuple_res",
                                                                                                    },
                                                                                                },
                                                                                                index: 3,
                                                                                                index_span: Span {
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1234,
                                                                                                    end: 1235,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1224,
                                                                                            end: 1235,
                                                                                            as_str(): "tuple_res.3",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                3,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1239,
                                                                                            end: 1240,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 1224,
                                                                            end: 1240,
                                                                            as_str(): "tuple_res.3 == 3",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 1204,
                                                                end: 1241,
                                                                as_str(): "(func_res == 3) && (tuple_res.3 == 3)",
                                                            },
                                                        },
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
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1259,
                                                                                            end: 1261,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1259,
                                                                                            end: 1261,
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
                                                                                        src (ptr): 0x00007fe0bd11adc0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                        ),
                                                                                        start: 1259,
                                                                                        end: 1261,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                            ),
                                                                            start: 1259,
                                                                            end: 1261,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Subfield(
                                                                                SubfieldExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0bd11adc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1246,
                                                                                                    end: 1256,
                                                                                                    as_str(): "struct_res",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1246,
                                                                                            end: 1256,
                                                                                            as_str(): "struct_res",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd11adc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                            ),
                                                                                            start: 1257,
                                                                                            end: 1258,
                                                                                            as_str(): "d",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1246,
                                                                                end: 1258,
                                                                                as_str(): "struct_res.d",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: Literal(
                                                                                Numeric(
                                                                                    3,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                                ),
                                                                                start: 1262,
                                                                                end: 1263,
                                                                                as_str(): "3",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd11adc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                                ),
                                                                start: 1246,
                                                                end: 1263,
                                                                as_str(): "struct_res.d == 3",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd11adc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                                    ),
                                                    start: 1204,
                                                    end: 1264,
                                                    as_str(): "(func_res == 3) && (tuple_res.3 == 3) && (struct_res.d == 3)",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bd11adc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                            ),
                                            start: 1197,
                                            end: 1264,
                                            as_str(): "return (func_res == 3) && (tuple_res.3 == 3) && (struct_res.d == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd11adc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                                    ),
                                    start: 1197,
                                    end: 1264,
                                    as_str(): "return (func_res == 3) && (tuple_res.3 == 3) && (struct_res.d == 3)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0bd11adc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                            ),
                            start: 235,
                            end: 1267,
                            as_str(): "{\n    let mut x: u64 = 0;\n\n    // function arguments evaluation\n    let func_res =\n        func(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                0\n            },\n            {\n                x = 3;\n                0\n            },\n            x\n        );\n\n    // tuple evaluation\n    x = 0;\n    let tuple_res =\n        (\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                1\n            },\n            {\n                x = 3;\n                2\n            },\n            x\n        );\n\n    // struct evaluation\n    x = 0;\n    let struct_res =\n        MyStruct {\n            a: {\n                x = 1;\n                0\n            },\n            b: {\n                x = 2;\n                1\n            },\n            c: {\n                x = 3;\n                2\n            },\n            d: x\n        };\n\n    return (func_res == 3) && (tuple_res.3 == 3) && (struct_res.d == 3);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0bd11adc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                        ),
                        start: 217,
                        end: 1267,
                        as_str(): "fn main() -> bool {\n    let mut x: u64 = 0;\n\n    // function arguments evaluation\n    let func_res =\n        func(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                0\n            },\n            {\n                x = 3;\n                0\n            },\n            x\n        );\n\n    // tuple evaluation\n    x = 0;\n    let tuple_res =\n        (\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                1\n            },\n            {\n                x = 3;\n                2\n            },\n            x\n        );\n\n    // struct evaluation\n    x = 0;\n    let struct_res =\n        MyStruct {\n            a: {\n                x = 1;\n                0\n            },\n            b: {\n                x = 2;\n                1\n            },\n            c: {\n                x = 3;\n                2\n            },\n            d: x\n        };\n\n    return (func_res == 3) && (tuple_res.3 == 3) && (struct_res.d == 3);\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0bd11adc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
                        ),
                        start: 230,
                        end: 234,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0bd11adc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRyFpSxY/left_to_right_func_args_evaluation/src/main.sw",
            ),
            start: 217,
            end: 1267,
            as_str(): "fn main() -> bool {\n    let mut x: u64 = 0;\n\n    // function arguments evaluation\n    let func_res =\n        func(\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                0\n            },\n            {\n                x = 3;\n                0\n            },\n            x\n        );\n\n    // tuple evaluation\n    x = 0;\n    let tuple_res =\n        (\n            {\n                x = 1;\n                0\n            },\n            {\n                x = 2;\n                1\n            },\n            {\n                x = 3;\n                2\n            },\n            x\n        );\n\n    // struct evaluation\n    x = 0;\n    let struct_res =\n        MyStruct {\n            a: {\n                x = 1;\n                0\n            },\n            b: {\n                x = 2;\n                1\n            },\n            c: {\n                x = 3;\n                2\n            },\n            d: x\n        };\n\n    return (func_res == 3) && (tuple_res.3 == 3) && (struct_res.d == 3);\n}",
        },
    },
]
