

TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 50,
            end: 58,
            as_str(): "sum_test",
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
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 97,
                                    end: 100,
                                    as_str(): "sum",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 109,
                                                    end: 110,
                                                    as_str(): "+",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 109,
                                                    end: 110,
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
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 109,
                                                end: 110,
                                                as_str(): "+",
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
                                                    src (ptr): 0x00007fe07c2eb040,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 124,
                                                    end: 128,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "core",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 106,
                                                                    as_str(): "+",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 106,
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 106,
                                                                as_str(): "+",
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
                                                                    src (ptr): 0x00007fe07c2eb040,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 124,
                                                                    end: 128,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 59,
                                                                            end: 60,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 103,
                                                                        end: 104,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 103,
                                                                    end: 104,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c2eb040,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 130,
                                                                    end: 135,
                                                                    as_str(): "other",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 66,
                                                                            end: 67,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 107,
                                                                        end: 108,
                                                                        as_str(): "b",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 107,
                                                                    end: 108,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13314,
                                                        Span {
                                                            src (ptr): 0x00007fe07c2eb040,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 117,
                                                            end: 185,
                                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 106,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 103,
                                                    end: 108,
                                                    as_str(): "a + b",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c2eb040,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 130,
                                                    end: 135,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 73,
                                                            end: 74,
                                                            as_str(): "c",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 112,
                                                        as_str(): "c",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 111,
                                                    end: 112,
                                                    as_str(): "c",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13315,
                                        Span {
                                            src (ptr): 0x00007fe07c2eb040,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 117,
                                            end: 185,
                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 109,
                                                end: 110,
                                                as_str(): "+",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 103,
                                    end: 112,
                                    as_str(): "a + b + c",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31631,
                            ),
                            type_ascription: TypeId(
                                31631,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 93,
                    end: 113,
                    as_str(): "let sum = a + b + c;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 97,
                                    end: 100,
                                    as_str(): "sum",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe073386450,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                ),
                                start: 118,
                                end: 121,
                                as_str(): "sum",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            31631,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 118,
                            end: 121,
                            as_str(): "sum",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 118,
                    end: 121,
                    as_str(): "sum",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 59,
                    end: 60,
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
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe073386450,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                ),
                start: 61,
                end: 64,
                as_str(): "u64",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 66,
                    end: 67,
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
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe073386450,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                ),
                start: 68,
                end: 71,
                as_str(): "u64",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 73,
                    end: 74,
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
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe073386450,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                ),
                start: 75,
                end: 78,
                as_str(): "u64",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe073386450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
        ),
        start: 47,
        end: 123,
        as_str(): "fn sum_test(a:u64, b:u64, c:u64) -> u64 {\n    let sum = a + b + c;\n    sum\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe073386450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
        ),
        start: 83,
        end: 86,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 128,
            end: 145,
            as_str(): "reassignment_test",
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
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 179,
                                    end: 180,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        2,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 183,
                                    end: 184,
                                    as_str(): "2",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31637,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 171,
                    end: 185,
                    as_str(): "let mut b = 2;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 146,
                                            end: 150,
                                            as_str(): "cond",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 193,
                                        end: 197,
                                        as_str(): "cond",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 193,
                                    end: 197,
                                    as_str(): "cond",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Reassignment(
                                                            TyReassignment {
                                                                lhs_base_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 208,
                                                                        end: 209,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                lhs_type: TypeId(
                                                                    21,
                                                                ),
                                                                lhs_indices: [],
                                                                rhs: TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        31643,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 212,
                                                                        end: 214,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31645,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 208,
                                                            end: 214,
                                                            as_str(): "b = 42",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 208,
                                                    end: 214,
                                                    as_str(): "b = 42",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    31647,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 198,
                                    end: 221,
                                    as_str(): "{\n        b = 42;\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: Expression(
                                                        TyExpression {
                                                            expression: Reassignment(
                                                                TyReassignment {
                                                                    lhs_base_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 237,
                                                                            end: 238,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    lhs_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    lhs_indices: [],
                                                                    rhs: TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31651,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 241,
                                                                            end: 242,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31653,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 237,
                                                                end: 242,
                                                                as_str(): "b = 5",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 237,
                                                        end: 242,
                                                        as_str(): "b = 5",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        31655,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 227,
                                        end: 249,
                                        as_str(): "{\n        b = 5;\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31656,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 190,
                            end: 249,
                            as_str(): "if cond {\n        b = 42;\n    } else {\n        b = 5;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 190,
                    end: 249,
                    as_str(): "if cond {\n        b = 42;\n    } else {\n        b = 5;\n    }",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 179,
                                    end: 180,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe073386450,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                ),
                                start: 255,
                                end: 256,
                                as_str(): "b",
                            },
                            mutability: Mutable,
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 255,
                            end: 256,
                            as_str(): "b",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 255,
                    end: 256,
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
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 146,
                    end: 150,
                    as_str(): "cond",
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
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fe073386450,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                ),
                start: 152,
                end: 156,
                as_str(): "bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe073386450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
        ),
        start: 125,
        end: 258,
        as_str(): "fn reassignment_test(cond: bool) -> u64 {\n    let mut b = 2;\n    if cond {\n        b = 42;\n    } else {\n        b = 5;\n    };\n    b\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe073386450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
        ),
        start: 161,
        end: 164,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 263,
            end: 272,
            as_str(): "loop_test",
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
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 310,
                                    end: 311,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 314,
                                    end: 315,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31658,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 302,
                    end: 316,
                    as_str(): "let mut b = 0;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: WhileLoop {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 329,
                                                    end: 330,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 329,
                                                    end: 330,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "lt",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 329,
                                                end: 330,
                                                as_str(): "<",
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
                                                    src (ptr): 0x00007fe07c2eb040,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 4000,
                                                    end: 4004,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 310,
                                                            end: 311,
                                                            as_str(): "b",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 327,
                                                        end: 328,
                                                        as_str(): "b",
                                                    },
                                                    mutability: Mutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 328,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c2eb040,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 4006,
                                                    end: 4011,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 273,
                                                            end: 283,
                                                            as_str(): "trip_count",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 331,
                                                        end: 341,
                                                        as_str(): "trip_count",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 331,
                                                    end: 341,
                                                    as_str(): "trip_count",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13318,
                                        Span {
                                            src (ptr): 0x00007fe07c2eb040,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3994,
                                            end: 4129,
                                            as_str(): "fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 329,
                                                end: 330,
                                                as_str(): "<",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 341,
                                    as_str(): "b < trip_count",
                                },
                            },
                            body: TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Expression(
                                            TyExpression {
                                                expression: Reassignment(
                                                    TyReassignment {
                                                        lhs_base_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 352,
                                                                end: 353,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        lhs_type: TypeId(
                                                            21,
                                                        ),
                                                        lhs_indices: [],
                                                        rhs: TyExpression {
                                                            expression: FunctionApplication {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 358,
                                                                                end: 359,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 358,
                                                                                end: 359,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 358,
                                                                            end: 359,
                                                                            as_str(): "+",
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
                                                                                src (ptr): 0x00007fe07c2eb040,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 124,
                                                                                end: 128,
                                                                                as_str(): "self",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        TyExpression {
                                                                            expression: VariableExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 310,
                                                                                        end: 311,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 356,
                                                                                    end: 357,
                                                                                    as_str(): "b",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 356,
                                                                                end: 357,
                                                                                as_str(): "b",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c2eb040,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                ),
                                                                                start: 130,
                                                                                end: 135,
                                                                                as_str(): "other",
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 360,
                                                                                end: 361,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13319,
                                                                    Span {
                                                                        src (ptr): 0x00007fe07c2eb040,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 185,
                                                                        as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                    },
                                                                ),
                                                                self_state_idx: None,
                                                                selector: None,
                                                                type_binding: Some(
                                                                    TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 358,
                                                                            end: 359,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 356,
                                                                end: 361,
                                                                as_str(): "b + 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31670,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 352,
                                                    end: 361,
                                                    as_str(): "b = b + 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 352,
                                            end: 361,
                                            as_str(): "b = b + 1",
                                        },
                                    },
                                ],
                            },
                        },
                        return_type: TypeId(
                            31672,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 321,
                            end: 368,
                            as_str(): "while b < trip_count {\n        b = b + 1;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 321,
                    end: 368,
                    as_str(): "while b < trip_count {\n        b = b + 1;\n    }",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [
                                    BaseIdent {
                                        name_override_opt: Some(
                                            "core",
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 375,
                                            end: 376,
                                            as_str(): "+",
                                        },
                                        is_raw_ident: false,
                                    },
                                    BaseIdent {
                                        name_override_opt: Some(
                                            "ops",
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 375,
                                            end: 376,
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
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 375,
                                        end: 376,
                                        as_str(): "+",
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
                                            src (ptr): 0x00007fe07c2eb040,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 124,
                                            end: 128,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 310,
                                                    end: 311,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 373,
                                                end: 374,
                                                as_str(): "b",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 373,
                                            end: 374,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe07c2eb040,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 130,
                                            end: 135,
                                            as_str(): "other",
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 377,
                                            end: 378,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13320,
                                Span {
                                    src (ptr): 0x00007fe07c2eb040,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                    ),
                                    start: 117,
                                    end: 185,
                                    as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 375,
                                        end: 376,
                                        as_str(): "+",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 373,
                            end: 378,
                            as_str(): "b + 1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 373,
                    end: 378,
                    as_str(): "b + 1",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 273,
                    end: 283,
                    as_str(): "trip_count",
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
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe073386450,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                ),
                start: 284,
                end: 287,
                as_str(): "u64",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe073386450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
        ),
        start: 260,
        end: 380,
        as_str(): "fn loop_test(trip_count:u64) -> u64 {\n    let mut b = 0;\n    while b < trip_count {\n        b = b + 1;\n    }\n    b + 1\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe073386450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
        ),
        start: 292,
        end: 295,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe073386450,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
            ),
            start: 385,
            end: 389,
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
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 409,
                                    end: 413,
                                    as_str(): "four",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        4,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 416,
                                    end: 417,
                                    as_str(): "4",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31677,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 405,
                    end: 418,
                    as_str(): "let four = 4;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 427,
                                    end: 432,
                                    as_str(): "three",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        3,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 435,
                                    end: 436,
                                    as_str(): "3",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31679,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 423,
                    end: 437,
                    as_str(): "let three = 3;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 446,
                                    end: 449,
                                    as_str(): "sum",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 457,
                                                    end: 458,
                                                    as_str(): "+",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 457,
                                                    end: 458,
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
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 457,
                                                end: 458,
                                                as_str(): "+",
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
                                                    src (ptr): 0x00007fe07c2eb040,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 124,
                                                    end: 128,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 409,
                                                            end: 413,
                                                            as_str(): "four",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 452,
                                                        end: 456,
                                                        as_str(): "four",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 452,
                                                    end: 456,
                                                    as_str(): "four",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c2eb040,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 130,
                                                    end: 135,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 427,
                                                            end: 432,
                                                            as_str(): "three",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 459,
                                                        end: 464,
                                                        as_str(): "three",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 459,
                                                    end: 464,
                                                    as_str(): "three",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13322,
                                        Span {
                                            src (ptr): 0x00007fe07c2eb040,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 117,
                                            end: 185,
                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 457,
                                                end: 458,
                                                as_str(): "+",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 452,
                                    end: 464,
                                    as_str(): "four + three",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31681,
                            ),
                            type_ascription: TypeId(
                                31681,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 442,
                    end: 465,
                    as_str(): "let sum = four + three;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 470,
                                        end: 476,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe08388ec70,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 481,
                                                            end: 483,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 481,
                                                            end: 483,
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
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 481,
                                                        end: 483,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 446,
                                                                    end: 449,
                                                                    as_str(): "sum",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 477,
                                                                end: 480,
                                                                as_str(): "sum",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31681,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 477,
                                                            end: 480,
                                                            as_str(): "sum",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                                7,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 484,
                                                            end: 485,
                                                            as_str(): "7",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fe07c2eb040,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 481,
                                                        end: 483,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 477,
                                            end: 485,
                                            as_str(): "sum == 7",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fe08388ec70,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 470,
                                        end: 476,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31689,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 470,
                            end: 486,
                            as_str(): "assert(sum == 7)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 470,
                    end: 486,
                    as_str(): "assert(sum == 7)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 493,
                                        end: 499,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe08388ec70,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 500,
                                            end: 504,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13327,
                                Span {
                                    src (ptr): 0x00007fe08388ec70,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 493,
                                        end: 499,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31692,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 493,
                            end: 505,
                            as_str(): "assert(true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 493,
                    end: 505,
                    as_str(): "assert(true)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 511,
                                        end: 517,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe08388ec70,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 532,
                                                            end: 534,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 532,
                                                            end: 534,
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
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 532,
                                                        end: 534,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 518,
                                                                        end: 527,
                                                                        as_str(): "loop_test",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 273,
                                                                            end: 283,
                                                                            as_str(): "trip_count",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 528,
                                                                            end: 530,
                                                                            as_str(): "10",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13330,
                                                                Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 260,
                                                                    end: 380,
                                                                    as_str(): "fn loop_test(trip_count:u64) -> u64 {\n    let mut b = 0;\n    while b < trip_count {\n        b = b + 1;\n    }\n    b + 1\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 518,
                                                                        end: 527,
                                                                        as_str(): "loop_test",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 518,
                                                            end: 531,
                                                            as_str(): "loop_test(10)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                                11,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 535,
                                                            end: 537,
                                                            as_str(): "11",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13331,
                                                Span {
                                                    src (ptr): 0x00007fe07c2eb040,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 532,
                                                        end: 534,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 518,
                                            end: 537,
                                            as_str(): "loop_test(10) == 11",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13332,
                                Span {
                                    src (ptr): 0x00007fe08388ec70,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 511,
                                        end: 517,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31700,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 511,
                            end: 538,
                            as_str(): "assert(loop_test(10) == 11)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 511,
                    end: 538,
                    as_str(): "assert(loop_test(10) == 11)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 544,
                                        end: 550,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe08388ec70,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 576,
                                                            end: 578,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 576,
                                                            end: 578,
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
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 576,
                                                        end: 578,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 551,
                                                                        end: 568,
                                                                        as_str(): "reassignment_test",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 146,
                                                                            end: 150,
                                                                            as_str(): "cond",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 569,
                                                                            end: 574,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13335,
                                                                Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 125,
                                                                    end: 258,
                                                                    as_str(): "fn reassignment_test(cond: bool) -> u64 {\n    let mut b = 2;\n    if cond {\n        b = 42;\n    } else {\n        b = 5;\n    };\n    b\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 551,
                                                                        end: 568,
                                                                        as_str(): "reassignment_test",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 551,
                                                            end: 575,
                                                            as_str(): "reassignment_test(false)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                                5,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 579,
                                                            end: 580,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13336,
                                                Span {
                                                    src (ptr): 0x00007fe07c2eb040,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 576,
                                                        end: 578,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 551,
                                            end: 580,
                                            as_str(): "reassignment_test(false) == 5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fe08388ec70,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 544,
                                        end: 550,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31707,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 544,
                            end: 581,
                            as_str(): "assert(reassignment_test(false) == 5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 544,
                    end: 581,
                    as_str(): "assert(reassignment_test(false) == 5)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 587,
                                        end: 593,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe08388ec70,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 612,
                                                            end: 614,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 612,
                                                            end: 614,
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
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 612,
                                                        end: 614,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 594,
                                                                        end: 602,
                                                                        as_str(): "sum_test",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 59,
                                                                            end: 60,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 603,
                                                                            end: 604,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 66,
                                                                            end: 67,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 606,
                                                                            end: 607,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 73,
                                                                            end: 74,
                                                                            as_str(): "c",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 609,
                                                                            end: 610,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13340,
                                                                Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 47,
                                                                    end: 123,
                                                                    as_str(): "fn sum_test(a:u64, b:u64, c:u64) -> u64 {\n    let sum = a + b + c;\n    sum\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 594,
                                                                        end: 602,
                                                                        as_str(): "sum_test",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 594,
                                                            end: 611,
                                                            as_str(): "sum_test(1, 2, 3)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                                6,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 615,
                                                            end: 616,
                                                            as_str(): "6",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13341,
                                                Span {
                                                    src (ptr): 0x00007fe07c2eb040,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 612,
                                                        end: 614,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 594,
                                            end: 616,
                                            as_str(): "sum_test(1, 2, 3) == 6",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13342,
                                Span {
                                    src (ptr): 0x00007fe08388ec70,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 587,
                                        end: 593,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31719,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 587,
                            end: 617,
                            as_str(): "assert(sum_test(1, 2, 3) == 6)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 587,
                    end: 617,
                    as_str(): "assert(sum_test(1, 2, 3) == 6)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 623,
                                        end: 629,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe08388ec70,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 651,
                                                            end: 653,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 651,
                                                            end: 653,
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
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 651,
                                                        end: 653,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 630,
                                                                        end: 638,
                                                                        as_str(): "sum_test",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 59,
                                                                            end: 60,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                30,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 639,
                                                                            end: 641,
                                                                            as_str(): "30",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 66,
                                                                            end: 67,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                20,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 643,
                                                                            end: 645,
                                                                            as_str(): "20",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 73,
                                                                            end: 74,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 647,
                                                                            end: 649,
                                                                            as_str(): "10",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13345,
                                                                Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 47,
                                                                    end: 123,
                                                                    as_str(): "fn sum_test(a:u64, b:u64, c:u64) -> u64 {\n    let sum = a + b + c;\n    sum\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 630,
                                                                        end: 638,
                                                                        as_str(): "sum_test",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 630,
                                                            end: 650,
                                                            as_str(): "sum_test(30, 20, 10)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                                60,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 654,
                                                            end: 656,
                                                            as_str(): "60",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13346,
                                                Span {
                                                    src (ptr): 0x00007fe07c2eb040,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 651,
                                                        end: 653,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 630,
                                            end: 656,
                                            as_str(): "sum_test(30, 20, 10) == 60",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13347,
                                Span {
                                    src (ptr): 0x00007fe08388ec70,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 623,
                                        end: 629,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31731,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 623,
                            end: 657,
                            as_str(): "assert(sum_test(30, 20, 10) == 60)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 623,
                    end: 657,
                    as_str(): "assert(sum_test(30, 20, 10) == 60)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 663,
                                        end: 669,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe08388ec70,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 688,
                                                            end: 690,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 688,
                                                            end: 690,
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
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 688,
                                                        end: 690,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 670,
                                                                        end: 678,
                                                                        as_str(): "sum_test",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 59,
                                                                            end: 60,
                                                                            as_str(): "a",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 679,
                                                                            end: 680,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 66,
                                                                            end: 67,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 682,
                                                                            end: 683,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 73,
                                                                            end: 74,
                                                                            as_str(): "c",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 685,
                                                                            end: 686,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13350,
                                                                Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 47,
                                                                    end: 123,
                                                                    as_str(): "fn sum_test(a:u64, b:u64, c:u64) -> u64 {\n    let sum = a + b + c;\n    sum\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 670,
                                                                        end: 678,
                                                                        as_str(): "sum_test",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 670,
                                                            end: 687,
                                                            as_str(): "sum_test(3, 2, 1)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                                6,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 691,
                                                            end: 692,
                                                            as_str(): "6",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13351,
                                                Span {
                                                    src (ptr): 0x00007fe07c2eb040,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 688,
                                                        end: 690,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 670,
                                            end: 692,
                                            as_str(): "sum_test(3, 2, 1) == 6",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13352,
                                Span {
                                    src (ptr): 0x00007fe08388ec70,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 663,
                                        end: 669,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31743,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 663,
                            end: 693,
                            as_str(): "assert(sum_test(3, 2, 1) == 6)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 663,
                    end: 693,
                    as_str(): "assert(sum_test(3, 2, 1) == 6)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 704,
                                    end: 707,
                                    as_str(): "res",
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
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 710,
                                                end: 727,
                                                as_str(): "reassignment_test",
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
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 146,
                                                    end: 150,
                                                    as_str(): "cond",
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
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 728,
                                                    end: 732,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13354,
                                        Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 125,
                                            end: 258,
                                            as_str(): "fn reassignment_test(cond: bool) -> u64 {\n    let mut b = 2;\n    if cond {\n        b = 42;\n    } else {\n        b = 5;\n    };\n    b\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 710,
                                                end: 727,
                                                as_str(): "reassignment_test",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 710,
                                    end: 733,
                                    as_str(): "reassignment_test(true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31744,
                            ),
                            type_ascription: TypeId(
                                31744,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 700,
                    end: 734,
                    as_str(): "let res = reassignment_test(true);",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 739,
                                        end: 745,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe08388ec70,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 750,
                                                            end: 752,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 750,
                                                            end: 752,
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
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 750,
                                                        end: 752,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 704,
                                                                    end: 707,
                                                                    as_str(): "res",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 746,
                                                                end: 749,
                                                                as_str(): "res",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31744,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 746,
                                                            end: 749,
                                                            as_str(): "res",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c2eb040,
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
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 753,
                                                            end: 755,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13356,
                                                Span {
                                                    src (ptr): 0x00007fe07c2eb040,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 750,
                                                        end: 752,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 746,
                                            end: 755,
                                            as_str(): "res == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13357,
                                Span {
                                    src (ptr): 0x00007fe08388ec70,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 739,
                                        end: 745,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31751,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 739,
                            end: 756,
                            as_str(): "assert(res == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 739,
                    end: 756,
                    as_str(): "assert(res == 42)",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe073386450,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                    ),
                                    start: 704,
                                    end: 707,
                                    as_str(): "res",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe073386450,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                ),
                                start: 762,
                                end: 765,
                                as_str(): "res",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            31744,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 762,
                            end: 765,
                            as_str(): "res",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe073386450,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                    ),
                    start: 762,
                    end: 765,
                    as_str(): "res",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe073386450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
        ),
        start: 382,
        end: 767,
        as_str(): "fn main() -> u64 {\n    let four = 4;\n    let three = 3;\n    let sum = four + three;\n    assert(sum == 7);\n\n    assert(true);\n    assert(loop_test(10) == 11);\n    assert(reassignment_test(false) == 5);\n    assert(sum_test(1, 2, 3) == 6);\n    assert(sum_test(30, 20, 10) == 60);\n    assert(sum_test(3, 2, 1) == 6);\n\n    let res = reassignment_test(true);\n    assert(res == 42);\n    res\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe073386450,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
        ),
        start: 395,
        end: 398,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

