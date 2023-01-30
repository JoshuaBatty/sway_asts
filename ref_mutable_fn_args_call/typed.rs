TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe05beff8e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
            ),
            start: 238,
            end: 241,
            as_str(): "foo",
        },
        is_raw_ident: false,
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
                                        src (ptr): 0x00007fe05beff8e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                        ),
                                        start: 264,
                                        end: 265,
                                        as_str(): "x",
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
                                                        src (ptr): 0x00007fe05beff8e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                        ),
                                                        start: 266,
                                                        end: 268,
                                                        as_str(): "+=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05beff8e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                        ),
                                                        start: 266,
                                                        end: 268,
                                                        as_str(): "+=",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            ],
                                            suffix: BaseIdent {
                                                name_override_opt: Some(
                                                    "add",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05beff8e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                    ),
                                                    start: 266,
                                                    end: 268,
                                                    as_str(): "+=",
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
                                                        src (ptr): 0x00007fe05bcb2040,
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
                                                                src (ptr): 0x00007fe05beff8e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                ),
                                                                start: 250,
                                                                end: 251,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 264,
                                                            end: 265,
                                                            as_str(): "x",
                                                        },
                                                        mutability: RefMutable,
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05beff8e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                        ),
                                                        start: 264,
                                                        end: 265,
                                                        as_str(): "x",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe05bcb2040,
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
                                                        src (ptr): 0x00007fe05beff8e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                        ),
                                                        start: 269,
                                                        end: 270,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            544,
                                            Span {
                                                src (ptr): 0x00007fe05bcb2040,
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
                                                    src (ptr): 0x00007fe05beff8e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                    ),
                                                    start: 266,
                                                    end: 268,
                                                    as_str(): "+=",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe05beff8e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                        ),
                                        start: 264,
                                        end: 270,
                                        as_str(): "x += 1",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            7260,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe05beff8e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                            ),
                            start: 264,
                            end: 270,
                            as_str(): "x += 1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe05beff8e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                    ),
                    start: 264,
                    end: 270,
                    as_str(): "x += 1",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe05beff8e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                    ),
                    start: 250,
                    end: 251,
                    as_str(): "x",
                },
                is_raw_ident: false,
            },
            is_reference: true,
            is_mutable: true,
            mutability_span: Span {
                src (ptr): 0x00007fe05beff8e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                ),
                start: 242,
                end: 249,
                as_str(): "ref mut",
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe05beff8e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                ),
                start: 253,
                end: 256,
                as_str(): "u64",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05beff8e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
        ),
        start: 235,
        end: 273,
        as_str(): "fn foo(ref mut x: u64) {\n    x += 1;\n}",
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
        src (ptr): 0x00007fe05beff8e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
        ),
        start: 235,
        end: 257,
        as_str(): "fn foo(ref mut x: u64)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe05beff8e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
            ),
            start: 278,
            end: 282,
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
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 306,
                                    end: 307,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 310,
                                    end: 311,
                                    as_str(): "1",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7263,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe05beff8e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                    ),
                    start: 298,
                    end: 312,
                    as_str(): "let mut x = 1;",
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
                                        src (ptr): 0x00007fe05beff8e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                        ),
                                        start: 317,
                                        end: 320,
                                        as_str(): "foo",
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
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 250,
                                            end: 251,
                                            as_str(): "x",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05beff8e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                    ),
                                                    start: 306,
                                                    end: 307,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe05beff8e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                ),
                                                start: 321,
                                                end: 322,
                                                as_str(): "x",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 321,
                                            end: 322,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                547,
                                Span {
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 235,
                                    end: 273,
                                    as_str(): "fn foo(ref mut x: u64) {\n    x += 1;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe05beff8e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                        ),
                                        start: 317,
                                        end: 320,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7267,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe05beff8e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                            ),
                            start: 317,
                            end: 323,
                            as_str(): "foo(x)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe05beff8e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                    ),
                    start: 317,
                    end: 323,
                    as_str(): "foo(x)",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05beff8e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                    ),
                                    start: 306,
                                    end: 307,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe05beff8e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                ),
                                start: 329,
                                end: 330,
                                as_str(): "x",
                            },
                            mutability: Mutable,
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe05beff8e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                            ),
                            start: 329,
                            end: 330,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe05beff8e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                    ),
                    start: 329,
                    end: 330,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05beff8e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
        ),
        start: 275,
        end: 332,
        as_str(): "fn main() -> u64 {\n    let mut x = 1;\n    foo(x);\n    x\n}",
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
        src (ptr): 0x00007fe05beff8e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
        ),
        start: 288,
        end: 291,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

