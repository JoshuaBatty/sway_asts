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
                            src (ptr): 0x00007fb14ca11210,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                            ),
                            start: 12,
                            end: 26,
                            as_str(): "get_array_pair",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Array(
                                            ArrayExpression {
                                                contents: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 59,
                                                                    end: 60,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 60,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 62,
                                                                    end: 63,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 62,
                                                            end: 63,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                ],
                                                length_span: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 58,
                                            end: 64,
                                            as_str(): "[a, b]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 58,
                                    end: 64,
                                    as_str(): "[a, b]",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb14ca11210,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                            ),
                            start: 52,
                            end: 66,
                            as_str(): "{\n    [a, b]\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 30,
                                    end: 31,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 34,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 33,
                                end: 34,
                                as_str(): "T",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 37,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 39,
                                        end: 40,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 39,
                                end: 40,
                                as_str(): "T",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb14ca11210,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                        ),
                        start: 9,
                        end: 66,
                        as_str(): "fn get_array_pair<T>(a: T, b: T) -> [T; 2] {\n    [a, b]\n}",
                    },
                    return_type: Array(
                        TypeArgument {
                            type_id: TypeId(
                                7249,
                            ),
                            initial_type_id: TypeId(
                                7249,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 46,
                                end: 47,
                                as_str(): "T",
                            },
                        },
                        Length {
                            val: 2,
                            span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 49,
                                end: 50,
                                as_str(): "2",
                            },
                        },
                    ),
                    type_parameters: [
                        T: TypeId(7250),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fb14ca11210,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                        ),
                        start: 45,
                        end: 51,
                        as_str(): "[T; 2]",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14ca11210,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
            ),
            start: 9,
            end: 66,
            as_str(): "fn get_array_pair<T>(a: T, b: T) -> [T; 2] {\n    [a, b]\n}",
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
                            src (ptr): 0x00007fb14ca11210,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                            ),
                            start: 71,
                            end: 85,
                            as_str(): "idx_array_pair",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 123,
                                                                end: 126,
                                                                as_str(): "ary",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 123,
                                                        end: 126,
                                                        as_str(): "ary",
                                                    },
                                                },
                                                index: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 127,
                                                                end: 130,
                                                                as_str(): "idx",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 127,
                                                        end: 130,
                                                        as_str(): "idx",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 123,
                                            end: 131,
                                            as_str(): "ary[idx]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 123,
                                    end: 131,
                                    as_str(): "ary[idx]",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb14ca11210,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                            ),
                            start: 117,
                            end: 133,
                            as_str(): "{\n    ary[idx]\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 89,
                                    end: 92,
                                    as_str(): "ary",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Array(
                                TypeArgument {
                                    type_id: TypeId(
                                        7251,
                                    ),
                                    initial_type_id: TypeId(
                                        7251,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 96,
                                        as_str(): "T",
                                    },
                                },
                                Length {
                                    val: 2,
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 98,
                                        end: 99,
                                        as_str(): "2",
                                    },
                                },
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 94,
                                end: 100,
                                as_str(): "[T; 2]",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 102,
                                    end: 105,
                                    as_str(): "idx",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 107,
                                end: 110,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb14ca11210,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                        ),
                        start: 68,
                        end: 133,
                        as_str(): "fn idx_array_pair<T>(ary: [T; 2], idx: u64) -> T {\n    ary[idx]\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 115,
                                end: 116,
                                as_str(): "T",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        T: TypeId(7252),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fb14ca11210,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                        ),
                        start: 115,
                        end: 116,
                        as_str(): "T",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14ca11210,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
            ),
            start: 68,
            end: 133,
            as_str(): "fn idx_array_pair<T>(ary: [T; 2], idx: u64) -> T {\n    ary[idx]\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14ca11210,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                            ),
                            start: 142,
                            end: 143,
                            as_str(): "S",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 153,
                                    end: 154,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Array(
                                TypeArgument {
                                    type_id: TypeId(
                                        7253,
                                    ),
                                    initial_type_id: TypeId(
                                        7253,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 157,
                                        end: 158,
                                        as_str(): "T",
                                    },
                                },
                                Length {
                                    val: 10,
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 160,
                                        end: 162,
                                        as_str(): "10",
                                    },
                                },
                            ),
                            span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 153,
                                end: 163,
                                as_str(): "a: [T; 10]",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb14ca11210,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                ),
                                start: 156,
                                end: 163,
                                as_str(): "[T; 10]",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7254),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb14ca11210,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                        ),
                        start: 135,
                        end: 166,
                        as_str(): "struct S<T> {\n    a: [T; 10],\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14ca11210,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
            ),
            start: 135,
            end: 166,
            as_str(): "struct S<T> {\n    a: [T; 10],\n}",
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
                            src (ptr): 0x00007fb14ca11210,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                            ),
                            start: 171,
                            end: 175,
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
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 196,
                                                    end: 203,
                                                    as_str(): "ary_u64",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        21,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 206,
                                                        end: 209,
                                                        as_str(): "u64",
                                                    },
                                                },
                                                Length {
                                                    val: 2,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 211,
                                                        end: 212,
                                                        as_str(): "2",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 205,
                                                    end: 213,
                                                    as_str(): "[u64; 2]",
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
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 216,
                                                                        end: 230,
                                                                        as_str(): "get_array_pair",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 216,
                                                                end: 230,
                                                                as_str(): "get_array_pair",
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
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 231,
                                                                    end: 232,
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
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 234,
                                                                    end: 235,
                                                                    as_str(): "2",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 216,
                                                    end: 236,
                                                    as_str(): "get_array_pair(1, 2)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 192,
                                    end: 237,
                                    as_str(): "let ary_u64: [u64; 2] = get_array_pair(1, 2);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 247,
                                                    end: 248,
                                                    as_str(): "s",
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
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 251,
                                                                        end: 252,
                                                                        as_str(): "S",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 251,
                                                                end: 252,
                                                                as_str(): "S",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 263,
                                                                        end: 264,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Array(
                                                                        ArrayExpression {
                                                                            contents: [
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ca11210,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 272,
                                                                                        as_str(): "0_u64",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            length_span: Some(
                                                                                Span {
                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 276,
                                                                                    as_str(): "10",
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 266,
                                                                        end: 277,
                                                                        as_str(): "[0_u64; 10]",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 263,
                                                                    end: 277,
                                                                    as_str(): "a: [0_u64; 10]",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 251,
                                                    end: 283,
                                                    as_str(): "S {\n        a: [0_u64; 10]\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 243,
                                    end: 284,
                                    as_str(): "let s = S {\n        a: [0_u64; 10]\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 293,
                                                    end: 294,
                                                    as_str(): "t",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: ArrayIndex(
                                                    ArrayIndexExpression {
                                                        prefix: Expression {
                                                            kind: Subfield(
                                                                SubfieldExpression {
                                                                    prefix: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ca11210,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                                    ),
                                                                                    start: 298,
                                                                                    end: 299,
                                                                                    as_str(): "s",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 298,
                                                                            end: 299,
                                                                            as_str(): "s",
                                                                        },
                                                                    },
                                                                    field_to_access: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ca11210,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                            ),
                                                                            start: 300,
                                                                            end: 301,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 298,
                                                                end: 301,
                                                                as_str(): "s.a",
                                                            },
                                                        },
                                                        index: Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    9,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 303,
                                                                end: 304,
                                                                as_str(): "9",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 297,
                                                    end: 305,
                                                    as_str(): "(s.a)[9]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 289,
                                    end: 306,
                                    as_str(): "let t = (s.a)[9];",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 316,
                                                    end: 324,
                                                    as_str(): "ary_bool",
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
                                                                        src (ptr): 0x00007fb14ca11210,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                        ),
                                                                        start: 327,
                                                                        end: 341,
                                                                        as_str(): "get_array_pair",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 341,
                                                                as_str(): "get_array_pair",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        false,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 342,
                                                                    end: 347,
                                                                    as_str(): "false",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 349,
                                                                    end: 353,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14ca11210,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 354,
                                                    as_str(): "get_array_pair(false, true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 312,
                                    end: 355,
                                    as_str(): "let ary_bool = get_array_pair(false, true);",
                                },
                            },
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
                                                                src (ptr): 0x00007fb14ca11210,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                ),
                                                                start: 360,
                                                                end: 374,
                                                                as_str(): "idx_array_pair",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 360,
                                                        end: 374,
                                                        as_str(): "idx_array_pair",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ca11210,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                                    ),
                                                                    start: 375,
                                                                    end: 383,
                                                                    as_str(): "ary_bool",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 375,
                                                            end: 383,
                                                            as_str(): "ary_bool",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                1,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ca11210,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                            ),
                                                            start: 385,
                                                            end: 386,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 360,
                                            end: 387,
                                            as_str(): "idx_array_pair(ary_bool, 1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 360,
                                    end: 387,
                                    as_str(): "idx_array_pair(ary_bool, 1)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb14ca11210,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                            ),
                            start: 186,
                            end: 389,
                            as_str(): "{\n    let ary_u64: [u64; 2] = get_array_pair(1, 2);\n\n    let s = S {\n        a: [0_u64; 10]\n    };\n    let t = (s.a)[9];\n\n    let ary_bool = get_array_pair(false, true);\n    idx_array_pair(ary_bool, 1)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb14ca11210,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                        ),
                        start: 168,
                        end: 389,
                        as_str(): "fn main() -> bool {\n    let ary_u64: [u64; 2] = get_array_pair(1, 2);\n\n    let s = S {\n        a: [0_u64; 10]\n    };\n    let t = (s.a)[9];\n\n    let ary_bool = get_array_pair(false, true);\n    idx_array_pair(ary_bool, 1)\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb14ca11210,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                        ),
                        start: 181,
                        end: 185,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14ca11210,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
            ),
            start: 168,
            end: 389,
            as_str(): "fn main() -> bool {\n    let ary_u64: [u64; 2] = get_array_pair(1, 2);\n\n    let s = S {\n        a: [0_u64; 10]\n    };\n    let t = (s.a)[9];\n\n    let ary_bool = get_array_pair(false, true);\n    idx_array_pair(ary_bool, 1)\n}",
        },
    },
]
