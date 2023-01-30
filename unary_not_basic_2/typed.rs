
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a158f2440,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
            ),
            start: 207,
            end: 215,
            as_str(): "and_true",
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
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 243,
                                    end: 244,
                                    as_str(): "y",
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
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 247,
                                                    end: 248,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 247,
                                                    end: 248,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "not",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 247,
                                                end: 248,
                                                as_str(): "!",
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
                                                    src (ptr): 0x00007f8a216622d0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2713,
                                                    end: 2717,
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
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 249,
                                                                    end: 250,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 249,
                                                                    end: 250,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: Some(
                                                                "not",
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 249,
                                                                end: 250,
                                                                as_str(): "!",
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
                                                                    src (ptr): 0x00007f8a216622d0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2713,
                                                                    end: 2717,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 216,
                                                                            end: 217,
                                                                            as_str(): "x",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a158f2440,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                        ),
                                                                        start: 250,
                                                                        end: 251,
                                                                        as_str(): "x",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 250,
                                                                    end: 251,
                                                                    as_str(): "x",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13314,
                                                        Span {
                                                            src (ptr): 0x00007f8a216622d0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2706,
                                                            end: 2760,
                                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 249,
                                                                end: 250,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 249,
                                                    end: 251,
                                                    as_str(): "!x",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13315,
                                        Span {
                                            src (ptr): 0x00007f8a216622d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 2706,
                                            end: 2760,
                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 247,
                                                end: 248,
                                                as_str(): "!",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 247,
                                    end: 251,
                                    as_str(): "! !x",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31631,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 239,
                    end: 252,
                    as_str(): "let y = ! !x;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: LazyOperator {
                            op: And,
                            lhs: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 216,
                                            end: 217,
                                            as_str(): "x",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a158f2440,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                        ),
                                        start: 257,
                                        end: 258,
                                        as_str(): "x",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 257,
                                    end: 258,
                                    as_str(): "x",
                                },
                            },
                            rhs: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 262,
                                    end: 266,
                                    as_str(): "true",
                                },
                            },
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a158f2440,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                            ),
                            start: 257,
                            end: 266,
                            as_str(): "x && true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 257,
                    end: 266,
                    as_str(): "x && true",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 216,
                    end: 217,
                    as_str(): "x",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007f8a28022a30,
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
                src (ptr): 0x00007f8a158f2440,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                ),
                start: 219,
                end: 223,
                as_str(): "bool",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a158f2440,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
        ),
        start: 204,
        end: 268,
        as_str(): "fn and_true(x: bool) -> bool {\n    let y = ! !x;\n    x && true\n}",
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
        src (ptr): 0x00007f8a158f2440,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
        ),
        start: 228,
        end: 232,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a158f2440,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
            ),
            start: 24,
            end: 28,
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
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 49,
                                    end: 50,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 63,
                                    as_str(): "true",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                71,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 52,
                                    end: 56,
                                    as_str(): "bool",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 45,
                    end: 64,
                    as_str(): "let a: bool = true;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 73,
                                    end: 74,
                                    as_str(): "b",
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
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 78,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 78,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "not",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 77,
                                                end: 78,
                                                as_str(): "!",
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
                                                    src (ptr): 0x00007f8a216622d0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2713,
                                                    end: 2717,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 49,
                                                            end: 50,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a158f2440,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                        ),
                                                        start: 78,
                                                        end: 79,
                                                        as_str(): "a",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 78,
                                                    end: 79,
                                                    as_str(): "a",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13317,
                                        Span {
                                            src (ptr): 0x00007f8a216622d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 2706,
                                            end: 2760,
                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 77,
                                                end: 78,
                                                as_str(): "!",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 77,
                                    end: 79,
                                    as_str(): "!a",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31635,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 69,
                    end: 80,
                    as_str(): "let b = !a;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 98,
                                    end: 99,
                                    as_str(): "c",
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
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 103,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 103,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "not",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 102,
                                                end: 103,
                                                as_str(): "!",
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
                                                    src (ptr): 0x00007f8a216622d0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2713,
                                                    end: 2717,
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
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 106,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 106,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: Some(
                                                                "not",
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 106,
                                                                as_str(): "!",
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
                                                                    src (ptr): 0x00007f8a216622d0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2713,
                                                                    end: 2717,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 73,
                                                                            end: 74,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a158f2440,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                        ),
                                                                        start: 106,
                                                                        end: 107,
                                                                        as_str(): "b",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 106,
                                                                    end: 107,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13318,
                                                        Span {
                                                            src (ptr): 0x00007f8a216622d0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2706,
                                                            end: 2760,
                                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 106,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 107,
                                                    as_str(): "!b",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13319,
                                        Span {
                                            src (ptr): 0x00007f8a216622d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 2706,
                                            end: 2760,
                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 102,
                                                end: 103,
                                                as_str(): "!",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 102,
                                    end: 108,
                                    as_str(): "!( !b)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31637,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 94,
                    end: 109,
                    as_str(): "let c = !( !b);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 127,
                                    end: 128,
                                    as_str(): "d",
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
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 131,
                                                    end: 132,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 131,
                                                    end: 132,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "not",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 131,
                                                end: 132,
                                                as_str(): "!",
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
                                                    src (ptr): 0x00007f8a216622d0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2713,
                                                    end: 2717,
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
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 134,
                                                                    end: 135,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 134,
                                                                    end: 135,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: Some(
                                                                "not",
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 135,
                                                                as_str(): "!",
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
                                                                    src (ptr): 0x00007f8a216622d0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2713,
                                                                    end: 2717,
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
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 136,
                                                                                    end: 137,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 136,
                                                                                    end: 137,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "not",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 136,
                                                                                end: 137,
                                                                                as_str(): "!",
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
                                                                                    src (ptr): 0x00007f8a216622d0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 2713,
                                                                                    end: 2717,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 98,
                                                                                            end: 99,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a158f2440,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                        ),
                                                                                        start: 137,
                                                                                        end: 138,
                                                                                        as_str(): "c",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 137,
                                                                                    end: 138,
                                                                                    as_str(): "c",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13320,
                                                                        Span {
                                                                            src (ptr): 0x00007f8a216622d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2706,
                                                                            end: 2760,
                                                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: Some(
                                                                        TypeBinding {
                                                                            inner: (),
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 136,
                                                                                end: 137,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 136,
                                                                    end: 138,
                                                                    as_str(): "!c",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13321,
                                                        Span {
                                                            src (ptr): 0x00007f8a216622d0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2706,
                                                            end: 2760,
                                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 135,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 134,
                                                    end: 138,
                                                    as_str(): "! !c",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13322,
                                        Span {
                                            src (ptr): 0x00007f8a216622d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 2706,
                                            end: 2760,
                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 131,
                                                end: 132,
                                                as_str(): "!",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 131,
                                    end: 139,
                                    as_str(): "!( ! !c)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31640,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 123,
                    end: 140,
                    as_str(): "let d = !( ! !c);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 157,
                                    end: 158,
                                    as_str(): "e",
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
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 163,
                                                    end: 164,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 163,
                                                    end: 164,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "not",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 163,
                                                end: 164,
                                                as_str(): "!",
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
                                                    src (ptr): 0x00007f8a216622d0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2713,
                                                    end: 2717,
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
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 165,
                                                                    end: 166,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 165,
                                                                    end: 166,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: Some(
                                                                "not",
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 165,
                                                                end: 166,
                                                                as_str(): "!",
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
                                                                    src (ptr): 0x00007f8a216622d0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2713,
                                                                    end: 2717,
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
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 167,
                                                                                    end: 168,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 167,
                                                                                    end: 168,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "not",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 167,
                                                                                end: 168,
                                                                                as_str(): "!",
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
                                                                                    src (ptr): 0x00007f8a216622d0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 2713,
                                                                                    end: 2717,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 127,
                                                                                            end: 128,
                                                                                            as_str(): "d",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a158f2440,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                        ),
                                                                                        start: 169,
                                                                                        end: 170,
                                                                                        as_str(): "d",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 169,
                                                                                    end: 170,
                                                                                    as_str(): "d",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13323,
                                                                        Span {
                                                                            src (ptr): 0x00007f8a216622d0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2706,
                                                                            end: 2760,
                                                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: Some(
                                                                        TypeBinding {
                                                                            inner: (),
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 167,
                                                                                end: 168,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 167,
                                                                    end: 171,
                                                                    as_str(): "!(d)",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13324,
                                                        Span {
                                                            src (ptr): 0x00007f8a216622d0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2706,
                                                            end: 2760,
                                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 165,
                                                                end: 166,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 165,
                                                    end: 171,
                                                    as_str(): "! !(d)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13325,
                                        Span {
                                            src (ptr): 0x00007f8a216622d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 2706,
                                            end: 2760,
                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 163,
                                                end: 164,
                                                as_str(): "!",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 163,
                                    end: 171,
                                    as_str(): "! ! !(d)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31644,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 153,
                    end: 173,
                    as_str(): "let e = ( ! ! !(d));",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: LazyOperator {
                            op: Or,
                            lhs: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 178,
                                                    end: 179,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 178,
                                                    end: 179,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "not",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 178,
                                                end: 179,
                                                as_str(): "!",
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
                                                    src (ptr): 0x00007f8a216622d0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2713,
                                                    end: 2717,
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
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 180,
                                                                end: 188,
                                                                as_str(): "and_true",
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
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 49,
                                                                            end: 50,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a158f2440,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                        ),
                                                                        start: 189,
                                                                        end: 190,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 189,
                                                                    end: 190,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13327,
                                                        Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 204,
                                                            end: 268,
                                                            as_str(): "fn and_true(x: bool) -> bool {\n    let y = ! !x;\n    x && true\n}",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 180,
                                                                end: 188,
                                                                as_str(): "and_true",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 180,
                                                    end: 191,
                                                    as_str(): "and_true(a)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13328,
                                        Span {
                                            src (ptr): 0x00007f8a216622d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 2706,
                                            end: 2760,
                                            as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 178,
                                                end: 179,
                                                as_str(): "!",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 178,
                                    end: 192,
                                    as_str(): "!(and_true(a))",
                                },
                            },
                            rhs: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 196,
                                    end: 200,
                                    as_str(): "true",
                                },
                            },
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a158f2440,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                            ),
                            start: 178,
                            end: 200,
                            as_str(): "!(and_true(a)) || true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a158f2440,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                    ),
                    start: 178,
                    end: 200,
                    as_str(): "!(and_true(a)) || true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a158f2440,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
        ),
        start: 21,
        end: 202,
        as_str(): "fn main() -> bool {\n    let a: bool = true;\n    let b = !a; // false\n    let c = !( !b); // false\n    let d = !( ! !c); // true\n    let e = ( ! ! !(d));\n    !(and_true(a)) || true\n}",
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
        src (ptr): 0x00007f8a158f2440,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
        ),
        start: 34,
        end: 38,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

