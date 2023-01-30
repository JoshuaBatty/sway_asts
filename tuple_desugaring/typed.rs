TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe03346fb00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
            ),
            start: 12,
            end: 24,
            as_str(): "gimme_a_pair",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Tuple {
                            fields: [
                                TyExpression {
                                    expression: Literal(
                                        U32(
                                            1,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        33,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe03346fb00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 52,
                                        as_str(): "1u32",
                                    },
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
                                        src (ptr): 0x00007fe03346fb00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                        ),
                                        start: 54,
                                        end: 58,
                                        as_str(): "2u64",
                                    },
                                },
                            ],
                        },
                        return_type: TypeId(
                            7255,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe03346fb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                            ),
                            start: 47,
                            end: 59,
                            as_str(): "(1u32, 2u64)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe03346fb00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                    ),
                    start: 47,
                    end: 59,
                    as_str(): "(1u32, 2u64)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe03346fb00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
        ),
        start: 9,
        end: 61,
        as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
    },
    attributes: {},
    return_type: TypeId(
        7253,
    ),
    initial_return_type: TypeId(
        7252,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe03346fb00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
        ),
        start: 30,
        end: 40,
        as_str(): "(u32, u64)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe03346fb00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
            ),
            start: 66,
            end: 70,
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
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 90,
                                    end: 91,
                                    as_str(): "x",
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
                                                src (ptr): 0x00007fe03346fb00,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                ),
                                                start: 94,
                                                end: 106,
                                                as_str(): "gimme_a_pair",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        546,
                                        Span {
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 61,
                                            as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe03346fb00,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                ),
                                                start: 94,
                                                end: 106,
                                                as_str(): "gimme_a_pair",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7258,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 108,
                                    as_str(): "gimme_a_pair()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7258,
                            ),
                            type_ascription: TypeId(
                                7257,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe03346fb00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                    ),
                    start: 86,
                    end: 109,
                    as_str(): "let x = gimme_a_pair();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 118,
                                    end: 119,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Declaration(
                                                    VariableDeclaration(
                                                        TyVariableDeclaration {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "__match_return_var_name_1",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 128,
                                                                    end: 129,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 90,
                                                                            end: 91,
                                                                            as_str(): "x",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe03346fb00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                        ),
                                                                        start: 128,
                                                                        end: 129,
                                                                        as_str(): "x",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7261,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 128,
                                                                    end: 129,
                                                                    as_str(): "x",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7261,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7260,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 209,
                                                    as_str(): "match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
                                                },
                                            },
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: IfExp {
                                                            condition: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 140,
                                                                                    end: 149,
                                                                                    as_str(): "(a, 3u64)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 140,
                                                                                    end: 149,
                                                                                    as_str(): "(a, 3u64)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                ),
                                                                                start: 140,
                                                                                end: 149,
                                                                                as_str(): "(a, 3u64)",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                    contract_call_params: {},
                                                                    arguments: [
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032e92990,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3022,
                                                                                    end: 3026,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: TupleElemAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_1",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 128,
                                                                                                    end: 129,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 128,
                                                                                                end: 129,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7263,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                            ),
                                                                                            start: 128,
                                                                                            end: 129,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                    },
                                                                                    elem_to_access_num: 1,
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7263,
                                                                                    ),
                                                                                    elem_to_access_span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 140,
                                                                                        end: 149,
                                                                                        as_str(): "(a, 3u64)",
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 140,
                                                                                    end: 149,
                                                                                    as_str(): "(a, 3u64)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032e92990,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3028,
                                                                                    end: 3033,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    U64(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 144,
                                                                                    end: 148,
                                                                                    as_str(): "3u64",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        547,
                                                                        Span {
                                                                            src (ptr): 0x00007fe032e92990,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3016,
                                                                            end: 3082,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 140,
                                                                    end: 149,
                                                                    as_str(): "(a, 3u64)",
                                                                },
                                                            },
                                                            then: TyExpression {
                                                                expression: CodeBlock(
                                                                    TyCodeBlock {
                                                                        contents: [
                                                                            TyAstNode {
                                                                                content: Declaration(
                                                                                    VariableDeclaration(
                                                                                        TyVariableDeclaration {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 141,
                                                                                                    end: 142,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            body: TyExpression {
                                                                                                expression: TupleElemAccess {
                                                                                                    prefix: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "__match_return_var_name_1",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 128,
                                                                                                                    end: 129,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 128,
                                                                                                                end: 129,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            7263,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 128,
                                                                                                            end: 129,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                    },
                                                                                                    elem_to_access_num: 0,
                                                                                                    resolved_type_of_parent: TypeId(
                                                                                                        7263,
                                                                                                    ),
                                                                                                    elem_to_access_span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 140,
                                                                                                        end: 149,
                                                                                                        as_str(): "(a, 3u64)",
                                                                                                    },
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 140,
                                                                                                    end: 149,
                                                                                                    as_str(): "(a, 3u64)",
                                                                                                },
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                            return_type: TypeId(
                                                                                                33,
                                                                                            ),
                                                                                            type_ascription: TypeId(
                                                                                                33,
                                                                                            ),
                                                                                            type_ascription_span: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 141,
                                                                                    end: 142,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            TyAstNode {
                                                                                content: ImplicitReturnExpression(
                                                                                    TyExpression {
                                                                                        expression: Tuple {
                                                                                            fields: [
                                                                                                TyExpression {
                                                                                                    expression: VariableExpression {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 141,
                                                                                                                end: 142,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 156,
                                                                                                            end: 157,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                        mutability: Immutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        33,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 156,
                                                                                                        end: 157,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: Literal(
                                                                                                        U32(
                                                                                                            7,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        33,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 159,
                                                                                                        end: 163,
                                                                                                        as_str(): "7u32",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7270,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                            ),
                                                                                            start: 155,
                                                                                            end: 164,
                                                                                            as_str(): "(a, 7u32)",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 155,
                                                                                    end: 164,
                                                                                    as_str(): "(a, 7u32)",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                return_type: TypeId(
                                                                    7271,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 153,
                                                                    end: 166,
                                                                    as_str(): "{ (a, 7u32) }",
                                                                },
                                                            },
                                                            else: Some(
                                                                TyExpression {
                                                                    expression: CodeBlock(
                                                                        TyCodeBlock {
                                                                            contents: [
                                                                                TyAstNode {
                                                                                    content: Declaration(
                                                                                        VariableDeclaration(
                                                                                            TyVariableDeclaration {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 177,
                                                                                                        end: 178,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                body: TyExpression {
                                                                                                    expression: TupleElemAccess {
                                                                                                        prefix: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 128,
                                                                                                                        end: 129,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 128,
                                                                                                                    end: 129,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7263,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 128,
                                                                                                                end: 129,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                        },
                                                                                                        elem_to_access_num: 0,
                                                                                                        resolved_type_of_parent: TypeId(
                                                                                                            7263,
                                                                                                        ),
                                                                                                        elem_to_access_span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 176,
                                                                                                            end: 182,
                                                                                                            as_str(): "(a, b)",
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        33,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 176,
                                                                                                        end: 182,
                                                                                                        as_str(): "(a, b)",
                                                                                                    },
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                                return_type: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                type_ascription: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                type_ascription_span: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 177,
                                                                                        end: 178,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                TyAstNode {
                                                                                    content: Declaration(
                                                                                        VariableDeclaration(
                                                                                            TyVariableDeclaration {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 180,
                                                                                                        end: 181,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                body: TyExpression {
                                                                                                    expression: TupleElemAccess {
                                                                                                        prefix: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 128,
                                                                                                                        end: 129,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 128,
                                                                                                                    end: 129,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7263,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 128,
                                                                                                                end: 129,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                        },
                                                                                                        elem_to_access_num: 1,
                                                                                                        resolved_type_of_parent: TypeId(
                                                                                                            7263,
                                                                                                        ),
                                                                                                        elem_to_access_span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 176,
                                                                                                            end: 182,
                                                                                                            as_str(): "(a, b)",
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 176,
                                                                                                        end: 182,
                                                                                                        as_str(): "(a, b)",
                                                                                                    },
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                type_ascription: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                type_ascription_span: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 180,
                                                                                        end: 181,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                },
                                                                                TyAstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        TyExpression {
                                                                                            expression: Tuple {
                                                                                                fields: [
                                                                                                    TyExpression {
                                                                                                        expression: Literal(
                                                                                                            U32(
                                                                                                                0,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            33,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 189,
                                                                                                            end: 193,
                                                                                                            as_str(): "0u32",
                                                                                                        },
                                                                                                    },
                                                                                                    TyExpression {
                                                                                                        expression: Literal(
                                                                                                            U32(
                                                                                                                9,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            33,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 195,
                                                                                                            end: 199,
                                                                                                            as_str(): "9u32",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                7279,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 188,
                                                                                                end: 200,
                                                                                                as_str(): "(0u32, 9u32)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 188,
                                                                                        end: 200,
                                                                                        as_str(): "(0u32, 9u32)",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        7280,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe03346fb00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                        ),
                                                                        start: 186,
                                                                        end: 202,
                                                                        as_str(): "{ (0u32, 9u32) }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            7281,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 153,
                                                            end: 166,
                                                            as_str(): "{ (a, 7u32) }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 209,
                                                    as_str(): "match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7282,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 122,
                                    end: 209,
                                    as_str(): "match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7282,
                            ),
                            type_ascription: TypeId(
                                7259,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe03346fb00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                    ),
                    start: 114,
                    end: 210,
                    as_str(): "let y = match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    };",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: CodeBlock(
                            TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Declaration(
                                            VariableDeclaration(
                                                TyVariableDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_2",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 222,
                                                            as_str(): "y",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 118,
                                                                    end: 119,
                                                                    as_str(): "y",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                ),
                                                                start: 221,
                                                                end: 222,
                                                                as_str(): "y",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            7284,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 222,
                                                            as_str(): "y",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        7284,
                                                    ),
                                                    type_ascription: TypeId(
                                                        7283,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 215,
                                            end: 255,
                                            as_str(): "match y {\n        (a, b) => { b },\n    }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: CodeBlock(
                                                    TyCodeBlock {
                                                        contents: [
                                                            TyAstNode {
                                                                content: Declaration(
                                                                    VariableDeclaration(
                                                                        TyVariableDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 234,
                                                                                    end: 235,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            body: TyExpression {
                                                                                expression: TupleElemAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_2",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 221,
                                                                                                    end: 222,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 221,
                                                                                                end: 222,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7286,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                            ),
                                                                                            start: 221,
                                                                                            end: 222,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                    },
                                                                                    elem_to_access_num: 0,
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7286,
                                                                                    ),
                                                                                    elem_to_access_span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 233,
                                                                                        end: 239,
                                                                                        as_str(): "(a, b)",
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    33,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 233,
                                                                                    end: 239,
                                                                                    as_str(): "(a, b)",
                                                                                },
                                                                            },
                                                                            mutability: Immutable,
                                                                            return_type: TypeId(
                                                                                33,
                                                                            ),
                                                                            type_ascription: TypeId(
                                                                                33,
                                                                            ),
                                                                            type_ascription_span: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 234,
                                                                    end: 235,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            TyAstNode {
                                                                content: Declaration(
                                                                    VariableDeclaration(
                                                                        TyVariableDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 237,
                                                                                    end: 238,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            body: TyExpression {
                                                                                expression: TupleElemAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_2",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 221,
                                                                                                    end: 222,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 221,
                                                                                                end: 222,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7286,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                            ),
                                                                                            start: 221,
                                                                                            end: 222,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                    },
                                                                                    elem_to_access_num: 1,
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7286,
                                                                                    ),
                                                                                    elem_to_access_span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 233,
                                                                                        end: 239,
                                                                                        as_str(): "(a, b)",
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    33,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 233,
                                                                                    end: 239,
                                                                                    as_str(): "(a, b)",
                                                                                },
                                                                            },
                                                                            mutability: Immutable,
                                                                            return_type: TypeId(
                                                                                33,
                                                                            ),
                                                                            type_ascription: TypeId(
                                                                                33,
                                                                            ),
                                                                            type_ascription_span: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 237,
                                                                    end: 238,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                            TyAstNode {
                                                                content: ImplicitReturnExpression(
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 237,
                                                                                    end: 238,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                ),
                                                                                start: 245,
                                                                                end: 246,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 245,
                                                                            end: 246,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 245,
                                                                    end: 246,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 243,
                                                    end: 248,
                                                    as_str(): "{ b }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 215,
                                            end: 255,
                                            as_str(): "match y {\n        (a, b) => { b },\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            33,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe03346fb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                            ),
                            start: 215,
                            end: 255,
                            as_str(): "match y {\n        (a, b) => { b },\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe03346fb00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                    ),
                    start: 215,
                    end: 255,
                    as_str(): "match y {\n        (a, b) => { b },\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe03346fb00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
        ),
        start: 63,
        end: 257,
        as_str(): "fn main() -> u32 {\n    let x = gimme_a_pair();\n    let y = match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    };\n    match y {\n        (a, b) => { b },\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        33,
    ),
    initial_return_type: TypeId(
        33,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe03346fb00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
        ),
        start: 76,
        end: 79,
        as_str(): "u32",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

