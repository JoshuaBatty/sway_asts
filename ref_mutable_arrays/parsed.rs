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
                            src (ptr): 0x00007fe05c5aa670,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                            ),
                            start: 12,
                            end: 16,
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
                                                    src (ptr): 0x00007fe05c5aa670,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 43,
                                                    as_str(): "arr",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        0,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        0,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5aa670,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 46,
                                                        end: 49,
                                                        as_str(): "u64",
                                                    },
                                                },
                                                Length {
                                                    val: 1,
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5aa670,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 51,
                                                        end: 52,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe05c5aa670,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 53,
                                                    as_str(): "[u64; 1]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5aa670,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                                    ),
                                                                    start: 57,
                                                                    end: 58,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05c5aa670,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                    ),
                                                    start: 56,
                                                    end: 59,
                                                    as_str(): "[1]",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 32,
                                    end: 60,
                                    as_str(): "let mut arr: [u64; 1] = [1];",
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
                                                                src (ptr): 0x00007fe05c5aa670,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 84,
                                                                as_str(): "takes_ref_mut_array",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5aa670,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 65,
                                                        end: 84,
                                                        as_str(): "takes_ref_mut_array",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5aa670,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                                    ),
                                                                    start: 85,
                                                                    end: 88,
                                                                    as_str(): "arr",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5aa670,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                            ),
                                                            start: 85,
                                                            end: 88,
                                                            as_str(): "arr",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5aa670,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                            ),
                                            start: 65,
                                            end: 89,
                                            as_str(): "takes_ref_mut_array(arr)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 89,
                                    as_str(): "takes_ref_mut_array(arr)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: ArrayIndex(
                                            ArrayIndexExpression {
                                                prefix: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5aa670,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                                ),
                                                                start: 95,
                                                                end: 98,
                                                                as_str(): "arr",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5aa670,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 95,
                                                        end: 98,
                                                        as_str(): "arr",
                                                    },
                                                },
                                                index: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            0,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5aa670,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 99,
                                                        end: 100,
                                                        as_str(): "0",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5aa670,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                            ),
                                            start: 95,
                                            end: 101,
                                            as_str(): "arr[0]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 101,
                                    as_str(): "arr[0]",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05c5aa670,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                            ),
                            start: 26,
                            end: 103,
                            as_str(): "{\n    let mut arr: [u64; 1] = [1];\n    takes_ref_mut_array(arr);\n    arr[0]\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe05c5aa670,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                        ),
                        start: 9,
                        end: 103,
                        as_str(): "fn main() -> u64 {\n    let mut arr: [u64; 1] = [1];\n    takes_ref_mut_array(arr);\n    arr[0]\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05c5aa670,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                        ),
                        start: 22,
                        end: 25,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05c5aa670,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
            ),
            start: 9,
            end: 103,
            as_str(): "fn main() -> u64 {\n    let mut arr: [u64; 1] = [1];\n    takes_ref_mut_array(arr);\n    arr[0]\n}",
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
                            src (ptr): 0x00007fe05c5aa670,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                            ),
                            start: 108,
                            end: 127,
                            as_str(): "takes_ref_mut_array",
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
                                                        kind: ArrayIndex(
                                                            ArrayIndexExpression {
                                                                prefix: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05c5aa670,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                                                ),
                                                                                start: 157,
                                                                                end: 160,
                                                                                as_str(): "arr",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5aa670,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                                        ),
                                                                        start: 157,
                                                                        end: 160,
                                                                        as_str(): "arr",
                                                                    },
                                                                },
                                                                index: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5aa670,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                                        ),
                                                                        start: 161,
                                                                        end: 162,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5aa670,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                            ),
                                                            start: 157,
                                                            end: 163,
                                                            as_str(): "arr[0]",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            10,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5aa670,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                                        ),
                                                        start: 166,
                                                        end: 168,
                                                        as_str(): "10",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5aa670,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                            ),
                                            start: 157,
                                            end: 168,
                                            as_str(): "arr[0] = 10",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 157,
                                    end: 168,
                                    as_str(): "arr[0] = 10",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05c5aa670,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                            ),
                            start: 151,
                            end: 171,
                            as_str(): "{\n    arr[0] = 10;\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05c5aa670,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                    ),
                                    start: 136,
                                    end: 139,
                                    as_str(): "arr",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: true,
                            is_mutable: true,
                            mutability_span: Span {
                                src (ptr): 0x00007fe05c5aa670,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                ),
                                start: 128,
                                end: 135,
                                as_str(): "ref mut",
                            },
                            type_info: Array(
                                TypeArgument {
                                    type_id: TypeId(
                                        0,
                                    ),
                                    initial_type_id: TypeId(
                                        0,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe05c5aa670,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                        ),
                                        start: 142,
                                        end: 145,
                                        as_str(): "u64",
                                    },
                                },
                                Length {
                                    val: 1,
                                    span: Span {
                                        src (ptr): 0x00007fe05c5aa670,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                        ),
                                        start: 147,
                                        end: 148,
                                        as_str(): "1",
                                    },
                                },
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe05c5aa670,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                                ),
                                start: 141,
                                end: 149,
                                as_str(): "[u64; 1]",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe05c5aa670,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                        ),
                        start: 105,
                        end: 171,
                        as_str(): "fn takes_ref_mut_array(ref mut arr: [u64; 1]) {\n    arr[0] = 10;\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05c5aa670,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
                        ),
                        start: 105,
                        end: 150,
                        as_str(): "fn takes_ref_mut_array(ref mut arr: [u64; 1])",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05c5aa670,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHntsYY/ref_mutable_arrays/src/main.sw",
            ),
            start: 105,
            end: 171,
            as_str(): "fn takes_ref_mut_array(ref mut arr: [u64; 1]) {\n    arr[0] = 10;\n}",
        },
    },
]
