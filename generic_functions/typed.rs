TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc097840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
            ),
            start: 12,
            end: 20,
            as_str(): "identity",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 24,
                                    end: 25,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 39,
                                end: 40,
                                as_str(): "x",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7258,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 39,
                            end: 40,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 39,
                    end: 40,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 24,
                    end: 25,
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
                7258,
            ),
            initial_type_id: TypeId(
                7259,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc097840,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                ),
                start: 27,
                end: 28,
                as_str(): "T",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc097840,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
        ),
        start: 9,
        end: 42,
        as_str(): "fn identity<T>(x: T) -> T {\n  x\n}",
    },
    attributes: {},
    return_type: TypeId(
        7258,
    ),
    initial_return_type: TypeId(
        7260,
    ),
    type_parameters: [
        T: TypeId(7258),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0fc097840,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
        ),
        start: 33,
        end: 34,
        as_str(): "T",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc097840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
            ),
            start: 47,
            end: 59,
            as_str(): "two_generics",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 72,
                                    end: 73,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 87,
                                end: 88,
                                as_str(): "b",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7263,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 87,
                            end: 88,
                            as_str(): "b",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 87,
                    end: 88,
                    as_str(): "b",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 66,
                    end: 67,
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
            type_id: TypeId(
                7262,
            ),
            initial_type_id: TypeId(
                7264,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc097840,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                ),
                start: 69,
                end: 70,
                as_str(): "A",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 72,
                    end: 73,
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
            type_id: TypeId(
                7263,
            ),
            initial_type_id: TypeId(
                7265,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc097840,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                ),
                start: 75,
                end: 76,
                as_str(): "B",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc097840,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
        ),
        start: 44,
        end: 90,
        as_str(): "fn two_generics<A, B>(a: A, b: B) -> B {\n  b\n}",
    },
    attributes: {},
    return_type: TypeId(
        7263,
    ),
    initial_return_type: TypeId(
        7266,
    ),
    type_parameters: [
        A: TypeId(7262),
        B: TypeId(7263),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0fc097840,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
        ),
        start: 81,
        end: 82,
        as_str(): "B",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc097840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
            ),
            start: 95,
            end: 109,
            as_str(): "three_generics",
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
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 150,
                                    end: 151,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 119,
                                            end: 120,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 157,
                                        end: 158,
                                        as_str(): "a",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7268,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 157,
                                    end: 158,
                                    as_str(): "a",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7268,
                            ),
                            type_ascription: TypeId(
                                7268,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 153,
                                    end: 154,
                                    as_str(): "A",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 146,
                    end: 159,
                    as_str(): "let a: A = a;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 125,
                                    end: 126,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 162,
                                end: 163,
                                as_str(): "b",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7269,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 162,
                            end: 163,
                            as_str(): "b",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 162,
                    end: 163,
                    as_str(): "b",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 119,
                    end: 120,
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
            type_id: TypeId(
                7268,
            ),
            initial_type_id: TypeId(
                7271,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc097840,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                ),
                start: 122,
                end: 123,
                as_str(): "A",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 125,
                    end: 126,
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
            type_id: TypeId(
                7269,
            ),
            initial_type_id: TypeId(
                7272,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc097840,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                ),
                start: 128,
                end: 129,
                as_str(): "B",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 131,
                    end: 132,
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
            type_id: TypeId(
                7270,
            ),
            initial_type_id: TypeId(
                7273,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc097840,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                ),
                start: 134,
                end: 135,
                as_str(): "C",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc097840,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
        ),
        start: 92,
        end: 165,
        as_str(): "fn three_generics<A, B, C>(a: A, b: B, c: C) -> B {\n  let a: A = a;\n  b\n}",
    },
    attributes: {},
    return_type: TypeId(
        7269,
    ),
    initial_return_type: TypeId(
        7274,
    ),
    type_parameters: [
        A: TypeId(7268),
        B: TypeId(7269),
        C: TypeId(7270),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0fc097840,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
        ),
        start: 140,
        end: 141,
        as_str(): "B",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc097840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
            ),
            start: 170,
            end: 174,
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
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 193,
                                    end: 194,
                                    as_str(): "a",
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
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 205,
                                                end: 213,
                                                as_str(): "identity",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 25,
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 214,
                                                    end: 218,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        548,
                                        Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 42,
                                            as_str(): "fn identity<T>(x: T) -> T {\n  x\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 205,
                                                end: 213,
                                                as_str(): "identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7277,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 205,
                                    end: 219,
                                    as_str(): "identity(true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7277,
                            ),
                            type_ascription: TypeId(
                                71,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 196,
                                    end: 200,
                                    as_str(): "bool",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 189,
                    end: 220,
                    as_str(): "let a: bool   = identity(true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 227,
                                    end: 228,
                                    as_str(): "b",
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
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 239,
                                                end: 247,
                                                as_str(): "identity",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 25,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 248,
                                                    end: 250,
                                                    as_str(): "10",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        553,
                                        Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 42,
                                            as_str(): "fn identity<T>(x: T) -> T {\n  x\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 239,
                                                end: 247,
                                                as_str(): "identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7279,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 239,
                                    end: 251,
                                    as_str(): "identity(10)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33,
                            ),
                            type_ascription: TypeId(
                                33,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 230,
                                    end: 233,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 223,
                    end: 252,
                    as_str(): "let b: u32    = identity(10);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 259,
                                    end: 260,
                                    as_str(): "c",
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
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 271,
                                                end: 279,
                                                as_str(): "identity",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 25,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        42,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 280,
                                                    end: 282,
                                                    as_str(): "42",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        561,
                                        Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 42,
                                            as_str(): "fn identity<T>(x: T) -> T {\n  x\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 271,
                                                end: 279,
                                                as_str(): "identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7283,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 271,
                                    end: 283,
                                    as_str(): "identity(42)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 262,
                                    end: 265,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 255,
                    end: 284,
                    as_str(): "let c: u64    = identity(42);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 291,
                                    end: 292,
                                    as_str(): "e",
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
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 303,
                                                end: 311,
                                                as_str(): "identity",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 25,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    String(
                                                        Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 313,
                                                            end: 316,
                                                            as_str(): "foo",
                                                        },
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    7291,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 312,
                                                    end: 317,
                                                    as_str(): "\"foo\"",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        563,
                                        Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 42,
                                            as_str(): "fn identity<T>(x: T) -> T {\n  x\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 303,
                                                end: 311,
                                                as_str(): "identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7289,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 303,
                                    end: 318,
                                    as_str(): "identity(\"foo\")",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7289,
                            ),
                            type_ascription: TypeId(
                                7288,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 294,
                                    end: 300,
                                    as_str(): "str[3]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 287,
                    end: 319,
                    as_str(): "let e: str[3] = identity(\"foo\");",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 328,
                                    as_str(): "f",
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
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 336,
                                                end: 348,
                                                as_str(): "two_generics",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 66,
                                                    end: 67,
                                                    as_str(): "a",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 349,
                                                    end: 353,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 72,
                                                    end: 73,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 355,
                                                    end: 357,
                                                    as_str(): "10",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        574,
                                        Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 44,
                                            end: 90,
                                            as_str(): "fn two_generics<A, B>(a: A, b: B) -> B {\n  b\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 336,
                                                end: 348,
                                                as_str(): "two_generics",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7293,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 336,
                                    end: 358,
                                    as_str(): "two_generics(true, 10)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 330,
                                    end: 333,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 323,
                    end: 359,
                    as_str(): "let f: u64 = two_generics(true, 10);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 366,
                                    end: 367,
                                    as_str(): "g",
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
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 378,
                                                end: 392,
                                                as_str(): "three_generics",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 120,
                                                    as_str(): "a",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 393,
                                                    end: 397,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 125,
                                                    end: 126,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    String(
                                                        Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 400,
                                                            end: 403,
                                                            as_str(): "foo",
                                                        },
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    7306,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 399,
                                                    end: 404,
                                                    as_str(): "\"foo\"",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 131,
                                                    end: 132,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 406,
                                                    end: 408,
                                                    as_str(): "10",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        576,
                                        Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 92,
                                            end: 165,
                                            as_str(): "fn three_generics<A, B, C>(a: A, b: B, c: C) -> B {\n  let a: A = a;\n  b\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc097840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                ),
                                                start: 378,
                                                end: 392,
                                                as_str(): "three_generics",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7302,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 378,
                                    end: 409,
                                    as_str(): "three_generics(true, \"foo\", 10)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7302,
                            ),
                            type_ascription: TypeId(
                                7300,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 369,
                                    end: 375,
                                    as_str(): "str[3]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 362,
                    end: 410,
                    as_str(): "let g: str[3] = three_generics(true, \"foo\", 10);",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 193,
                                    end: 194,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 414,
                                end: 415,
                                as_str(): "a",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7277,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 414,
                            end: 415,
                            as_str(): "a",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc097840,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                    ),
                    start: 414,
                    end: 415,
                    as_str(): "a",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc097840,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
        ),
        start: 167,
        end: 418,
        as_str(): "fn main() -> bool {\n  let a: bool   = identity(true);\n  let b: u32    = identity(10);\n  let c: u64    = identity(42);\n  let e: str[3] = identity(\"foo\");\n\n  let f: u64 = two_generics(true, 10);\n  let g: str[3] = three_generics(true, \"foo\", 10);\n\n  a\n\n}",
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
        src (ptr): 0x00007fe0fc097840,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
        ),
        start: 180,
        end: 184,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

