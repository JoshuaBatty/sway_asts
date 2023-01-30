TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f8c2c380,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
            ),
            start: 16,
            end: 30,
            as_str(): "DoubleIdentity",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 41,
                    end: 46,
                    as_str(): "first",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7257,
            ),
            initial_type_id: TypeId(
                7259,
            ),
            span: Span {
                src (ptr): 0x00007fe0f8c2c380,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                ),
                start: 41,
                end: 49,
                as_str(): "first: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0f8c2c380,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                ),
                start: 48,
                end: 49,
                as_str(): "T",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 53,
                    end: 59,
                    as_str(): "second",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7258,
            ),
            initial_type_id: TypeId(
                7260,
            ),
            span: Span {
                src (ptr): 0x00007fe0f8c2c380,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                ),
                start: 53,
                end: 62,
                as_str(): "second: F",
            },
            type_span: Span {
                src (ptr): 0x00007fe0f8c2c380,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                ),
                start: 61,
                end: 62,
                as_str(): "F",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7257),
        F: TypeId(7258),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0f8c2c380,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
        ),
        start: 9,
        end: 64,
        as_str(): "struct DoubleIdentity<T, F> {\n  first: T,\n  second: F\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f8c2c380,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
            ),
            start: 69,
            end: 84,
            as_str(): "double_identity",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructExpression {
                            struct_name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 16,
                                    end: 30,
                                    as_str(): "DoubleIdentity",
                                },
                                is_raw_ident: false,
                            },
                            fields: [
                                TyStructExpressionField {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 152,
                                            end: 157,
                                            as_str(): "first",
                                        },
                                        is_raw_ident: false,
                                    },
                                    value: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 91,
                                                    end: 92,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "x",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7262,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 159,
                                            end: 160,
                                            as_str(): "x",
                                        },
                                    },
                                },
                                TyStructExpressionField {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 166,
                                            end: 172,
                                            as_str(): "second",
                                        },
                                        is_raw_ident: false,
                                    },
                                    value: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 97,
                                                    end: 98,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 174,
                                                end: 175,
                                                as_str(): "y",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7263,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 174,
                                            end: 175,
                                            as_str(): "y",
                                        },
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 131,
                                end: 145,
                                as_str(): "DoubleIdentity",
                            },
                        },
                        return_type: TypeId(
                            7271,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f8c2c380,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                            ),
                            start: 131,
                            end: 179,
                            as_str(): "DoubleIdentity {\n    first: x,\n    second: y\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 131,
                    end: 179,
                    as_str(): "DoubleIdentity {\n    first: x,\n    second: y\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 91,
                    end: 92,
                    as_str(): "x",
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
            type_id: TypeId(
                7262,
            ),
            initial_type_id: TypeId(
                7264,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f8c2c380,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                ),
                start: 94,
                end: 95,
                as_str(): "T",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 97,
                    end: 98,
                    as_str(): "y",
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
            type_id: TypeId(
                7263,
            ),
            initial_type_id: TypeId(
                7265,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f8c2c380,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                ),
                start: 100,
                end: 101,
                as_str(): "F",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f8c2c380,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
        ),
        start: 66,
        end: 181,
        as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        7267,
    ),
    initial_return_type: TypeId(
        7266,
    ),
    type_parameters: [
        T: TypeId(7262),
        F: TypeId(7263),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0f8c2c380,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
        ),
        start: 106,
        end: 126,
        as_str(): "DoubleIdentity<T, F>",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f8c2c380,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
            ),
            start: 186,
            end: 190,
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
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 209,
                                    end: 217,
                                    as_str(): "double_a",
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
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 220,
                                                end: 235,
                                                as_str(): "double_identity",
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 91,
                                                    end: 92,
                                                    as_str(): "x",
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 236,
                                                    end: 240,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 97,
                                                    end: 98,
                                                    as_str(): "y",
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 242,
                                                    end: 246,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        547,
                                        Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 181,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 220,
                                                end: 235,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7278,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 220,
                                    end: 247,
                                    as_str(): "double_identity(true, true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7278,
                            ),
                            type_ascription: TypeId(
                                7275,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 205,
                    end: 248,
                    as_str(): "let double_a = double_identity(true, true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 255,
                                    end: 263,
                                    as_str(): "double_b",
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
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 266,
                                                end: 281,
                                                as_str(): "double_identity",
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 91,
                                                    end: 92,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 282,
                                                    end: 287,
                                                    as_str(): "10u32",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 97,
                                                    end: 98,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        43,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 289,
                                                    end: 294,
                                                    as_str(): "43u64",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        555,
                                        Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 181,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 266,
                                                end: 281,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7285,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 266,
                                    end: 295,
                                    as_str(): "double_identity(10u32, 43u64)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7285,
                            ),
                            type_ascription: TypeId(
                                7282,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 251,
                    end: 296,
                    as_str(): "let double_b = double_identity(10u32, 43u64);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 333,
                                    end: 341,
                                    as_str(): "double_a",
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
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 372,
                                                end: 387,
                                                as_str(): "double_identity",
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 91,
                                                    end: 92,
                                                    as_str(): "x",
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 388,
                                                    end: 392,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 97,
                                                    end: 98,
                                                    as_str(): "y",
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 394,
                                                    end: 398,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        557,
                                        Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 181,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 372,
                                                end: 387,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7295,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 372,
                                    end: 399,
                                    as_str(): "double_identity(true, true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7295,
                            ),
                            type_ascription: TypeId(
                                7292,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 343,
                                    end: 369,
                                    as_str(): "DoubleIdentity<bool, bool>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 329,
                    end: 400,
                    as_str(): "let double_a: DoubleIdentity<bool, bool> = double_identity(true, true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 407,
                                    end: 415,
                                    as_str(): "double_b",
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
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 444,
                                                end: 459,
                                                as_str(): "double_identity",
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 91,
                                                    end: 92,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 460,
                                                    end: 465,
                                                    as_str(): "10u32",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 97,
                                                    end: 98,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        43,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 467,
                                                    end: 472,
                                                    as_str(): "43u64",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        577,
                                        Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 181,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f8c2c380,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                ),
                                                start: 444,
                                                end: 459,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7305,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 444,
                                    end: 473,
                                    as_str(): "double_identity(10u32, 43u64)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7305,
                            ),
                            type_ascription: TypeId(
                                7300,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 417,
                                    end: 441,
                                    as_str(): "DoubleIdentity<u32, u64>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 403,
                    end: 474,
                    as_str(): "let double_b: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructFieldAccess {
                            prefix: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 333,
                                            end: 341,
                                            as_str(): "double_a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 478,
                                        end: 486,
                                        as_str(): "double_a",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7295,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 478,
                                    end: 486,
                                    as_str(): "double_a",
                                },
                            },
                            field_to_access: TyStructField {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 46,
                                        as_str(): "first",
                                    },
                                    is_raw_ident: false,
                                },
                                type_id: TypeId(
                                    7293,
                                ),
                                initial_type_id: TypeId(
                                    7259,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 41,
                                    end: 49,
                                    as_str(): "first: T",
                                },
                                type_span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 48,
                                    end: 49,
                                    as_str(): "T",
                                },
                                attributes: {},
                            },
                            field_instantiation_span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 487,
                                end: 492,
                                as_str(): "first",
                            },
                            resolved_type_of_parent: TypeId(
                                7295,
                            ),
                        },
                        return_type: TypeId(
                            7293,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f8c2c380,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                            ),
                            start: 478,
                            end: 492,
                            as_str(): "double_a.first",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f8c2c380,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                    ),
                    start: 478,
                    end: 492,
                    as_str(): "double_a.first",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f8c2c380,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
        ),
        start: 183,
        end: 494,
        as_str(): "fn main() -> bool {\n  let double_a = double_identity(true, true);\n  let double_b = double_identity(10u32, 43u64);\n\n  // for testing annotations\n  let double_a: DoubleIdentity<bool, bool> = double_identity(true, true);\n  let double_b: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);\n\n  double_a.first\n}",
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
        src (ptr): 0x00007fe0f8c2c380,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
        ),
        start: 196,
        end: 200,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

