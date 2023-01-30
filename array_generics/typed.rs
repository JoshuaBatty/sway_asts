TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Array {
                            contents: [
                                TyExpression {
                                    expression: VariableExpression {
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
                                        span: Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 60,
                                            as_str(): "a",
                                        },
                                        mutability: Immutable,
                                    },
                                    return_type: TypeId(
                                        7258,
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
                                TyExpression {
                                    expression: VariableExpression {
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
                                        span: Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 62,
                                            end: 63,
                                            as_str(): "b",
                                        },
                                        mutability: Immutable,
                                    },
                                    return_type: TypeId(
                                        7258,
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
                        },
                        return_type: TypeId(
                            7266,
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
    },
    parameters: [
        TyFunctionParameter {
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
            type_id: TypeId(
                7258,
            ),
            initial_type_id: TypeId(
                7259,
            ),
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
        TyFunctionParameter {
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
            type_id: TypeId(
                7258,
            ),
            initial_type_id: TypeId(
                7260,
            ),
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
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14ca11210,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
        ),
        start: 9,
        end: 66,
        as_str(): "fn get_array_pair<T>(a: T, b: T) -> [T; 2] {\n    [a, b]\n}",
    },
    attributes: {},
    return_type: TypeId(
        7262,
    ),
    initial_return_type: TypeId(
        7261,
    ),
    type_parameters: [
        T: TypeId(7258),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: ArrayIndex {
                            prefix: TyExpression {
                                expression: VariableExpression {
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
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 123,
                                        end: 126,
                                        as_str(): "ary",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7274,
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
                            index: TyExpression {
                                expression: VariableExpression {
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
                                    span: Span {
                                        src (ptr): 0x00007fb14ca11210,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                        ),
                                        start: 127,
                                        end: 130,
                                        as_str(): "idx",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    21,
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
                        return_type: TypeId(
                            7268,
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
    },
    parameters: [
        TyFunctionParameter {
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
            type_id: TypeId(
                7270,
            ),
            initial_type_id: TypeId(
                7269,
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
        TyFunctionParameter {
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
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
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
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14ca11210,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
        ),
        start: 68,
        end: 133,
        as_str(): "fn idx_array_pair<T>(ary: [T; 2], idx: u64) -> T {\n    ary[idx]\n}",
    },
    attributes: {},
    return_type: TypeId(
        7268,
    ),
    initial_return_type: TypeId(
        7271,
    ),
    type_parameters: [
        T: TypeId(7268),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyStructDeclaration {
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
    fields: [
        TyStructField {
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
            type_id: TypeId(
                7277,
            ),
            initial_type_id: TypeId(
                7276,
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
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7275),
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
    attributes: {},
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
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
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
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
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
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
                                        ),
                                        (
                                            BaseIdent {
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
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        2,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
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
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        551,
                                        Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 66,
                                            as_str(): "fn get_array_pair<T>(a: T, b: T) -> [T; 2] {\n    [a, b]\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
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
                                    ),
                                },
                                return_type: TypeId(
                                    7289,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7289,
                            ),
                            type_ascription: TypeId(
                                7280,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
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
                                    fields: [
                                        TyStructExpressionField {
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
                                            value: TyExpression {
                                                expression: Array {
                                                    contents: [
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
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
                                                },
                                                return_type: TypeId(
                                                    7307,
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
                                        },
                                    ],
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
                                return_type: TypeId(
                                    7294,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7294,
                            ),
                            type_ascription: TypeId(
                                7290,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: ArrayIndex {
                                    prefix: TyExpression {
                                        expression: StructFieldAccess {
                                            prefix: TyExpression {
                                                expression: VariableExpression {
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
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ca11210,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                        ),
                                                        start: 298,
                                                        end: 299,
                                                        as_str(): "s",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7294,
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
                                            field_to_access: TyStructField {
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
                                                type_id: TypeId(
                                                    7293,
                                                ),
                                                initial_type_id: TypeId(
                                                    7276,
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
                                                attributes: {},
                                            },
                                            field_instantiation_span: Span {
                                                src (ptr): 0x00007fb14ca11210,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                ),
                                                start: 300,
                                                end: 301,
                                                as_str(): "a",
                                            },
                                            resolved_type_of_parent: TypeId(
                                                7294,
                                            ),
                                        },
                                        return_type: TypeId(
                                            7312,
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
                                    index: TyExpression {
                                        expression: Literal(
                                            U64(
                                                9,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            7313,
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
                                return_type: TypeId(
                                    7292,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7308,
                            ),
                            type_ascription: TypeId(
                                7308,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
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
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
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
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
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
                                        ),
                                        (
                                            BaseIdent {
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
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
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
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        553,
                                        Span {
                                            src (ptr): 0x00007fb14ca11210,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 66,
                                            as_str(): "fn get_array_pair<T>(a: T, b: T) -> [T; 2] {\n    [a, b]\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
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
                                    ),
                                },
                                return_type: TypeId(
                                    7320,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7320,
                            ),
                            type_ascription: TypeId(
                                7314,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
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
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: VariableExpression {
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
                                            span: Span {
                                                src (ptr): 0x00007fb14ca11210,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                                ),
                                                start: 375,
                                                end: 383,
                                                as_str(): "ary_bool",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7325,
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
                                ),
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: Literal(
                                            U64(
                                                1,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
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
                                ),
                            ],
                            function_decl_id: DeclId(
                                555,
                                Span {
                                    src (ptr): 0x00007fb14ca11210,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
                                    ),
                                    start: 68,
                                    end: 133,
                                    as_str(): "fn idx_array_pair<T>(ary: [T; 2], idx: u64) -> T {\n    ary[idx]\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            7321,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14ca11210,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIROQlN6X/array_generics/src/main.sw",
        ),
        start: 168,
        end: 389,
        as_str(): "fn main() -> bool {\n    let ary_u64: [u64; 2] = get_array_pair(1, 2);\n\n    let s = S {\n        a: [0_u64; 10]\n    };\n    let t = (s.a)[9];\n\n    let ary_bool = get_array_pair(false, true);\n    idx_array_pair(ary_bool, 1)\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

