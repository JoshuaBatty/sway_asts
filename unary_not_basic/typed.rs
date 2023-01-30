
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a1239c1f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                    src (ptr): 0x00007f8a1239c1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                src (ptr): 0x00007f8a1239c1f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a19328bf0,
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                            ),
                                                            start: 49,
                                                            end: 50,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1239c1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 78,
                                                    end: 79,
                                                    as_str(): "a",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13314,
                                        Span {
                                            src (ptr): 0x00007f8a19328bf0,
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
                                                src (ptr): 0x00007f8a1239c1f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                31631,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a1239c1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                src (ptr): 0x00007f8a1239c1f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a19328bf0,
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
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 105,
                                                                    as_str(): "!",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 105,
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 105,
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
                                                                    src (ptr): 0x00007f8a19328bf0,
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
                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                            ),
                                                                            start: 73,
                                                                            end: 74,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1239c1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                        ),
                                                                        start: 105,
                                                                        end: 106,
                                                                        as_str(): "b",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 106,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13315,
                                                        Span {
                                                            src (ptr): 0x00007f8a19328bf0,
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 105,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 104,
                                                    end: 106,
                                                    as_str(): "!b",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13316,
                                        Span {
                                            src (ptr): 0x00007f8a19328bf0,
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
                                                src (ptr): 0x00007f8a1239c1f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                    ),
                                    start: 102,
                                    end: 106,
                                    as_str(): "! !b",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31633,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a1239c1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                    ),
                    start: 94,
                    end: 107,
                    as_str(): "let c = ! !b;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                    ),
                                    start: 125,
                                    end: 126,
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 129,
                                                    end: 130,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 129,
                                                    end: 130,
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
                                                src (ptr): 0x00007f8a1239c1f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                ),
                                                start: 129,
                                                end: 130,
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
                                                    src (ptr): 0x00007f8a19328bf0,
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
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                    src (ptr): 0x00007f8a19328bf0,
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
                                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 133,
                                                                                    end: 134,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 133,
                                                                                    end: 134,
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                ),
                                                                                start: 133,
                                                                                end: 134,
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
                                                                                    src (ptr): 0x00007f8a19328bf0,
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
                                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                            ),
                                                                                            start: 98,
                                                                                            end: 99,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a1239c1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 134,
                                                                                        end: 135,
                                                                                        as_str(): "c",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 135,
                                                                                    as_str(): "c",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13317,
                                                                        Span {
                                                                            src (ptr): 0x00007f8a19328bf0,
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                ),
                                                                                start: 133,
                                                                                end: 134,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                    ),
                                                                    start: 133,
                                                                    end: 135,
                                                                    as_str(): "!c",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13318,
                                                        Span {
                                                            src (ptr): 0x00007f8a19328bf0,
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 131,
                                                    end: 135,
                                                    as_str(): "! !c",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13319,
                                        Span {
                                            src (ptr): 0x00007f8a19328bf0,
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
                                                src (ptr): 0x00007f8a1239c1f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                ),
                                                start: 129,
                                                end: 130,
                                                as_str(): "!",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                    ),
                                    start: 129,
                                    end: 135,
                                    as_str(): "! ! !c",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31636,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a1239c1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                    ),
                    start: 121,
                    end: 136,
                    as_str(): "let d = ! ! !c;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                    ),
                                    start: 125,
                                    end: 126,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007f8a1239c1f0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                ),
                                start: 149,
                                end: 150,
                                as_str(): "d",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a1239c1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                            ),
                            start: 149,
                            end: 150,
                            as_str(): "d",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a1239c1f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                    ),
                    start: 149,
                    end: 150,
                    as_str(): "d",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a1239c1f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
        ),
        start: 21,
        end: 152,
        as_str(): "fn main() -> bool {\n    let a: bool = true;\n    let b = !a; // false\n    let c = ! !b; // false\n    let d = ! ! !c; // true\n    d\n}",
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
        src (ptr): 0x00007f8a1239c1f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
        ),
        start: 34,
        end: 38,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

