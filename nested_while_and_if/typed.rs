

TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe07c8bc640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
            ),
            start: 51,
            end: 54,
            as_str(): "foo",
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
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 100,
                                    as_str(): "index",
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
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 103,
                                    end: 104,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31631,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 87,
                    end: 105,
                    as_str(): "let mut index = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 118,
                                    end: 121,
                                    as_str(): "sum",
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
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 124,
                                    end: 125,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31633,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 110,
                    end: 126,
                    as_str(): "let mut sum = 0;",
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
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 144,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 144,
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
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 143,
                                                end: 144,
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
                                                    src (ptr): 0x00007fe08b1c9ee0,
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
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 95,
                                                            end: 100,
                                                            as_str(): "index",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 137,
                                                        end: 142,
                                                        as_str(): "index",
                                                    },
                                                    mutability: Mutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 137,
                                                    end: 142,
                                                    as_str(): "index",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1c9ee0,
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
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 66,
                                                            end: 67,
                                                            as_str(): "n",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 145,
                                                        end: 146,
                                                        as_str(): "n",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 145,
                                                    end: 146,
                                                    as_str(): "n",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13314,
                                        Span {
                                            src (ptr): 0x00007fe08b1c9ee0,
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
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 143,
                                                end: 144,
                                                as_str(): "<",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 137,
                                    end: 146,
                                    as_str(): "index < n",
                                },
                            },
                            body: TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Expression(
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
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 170,
                                                                            end: 172,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 170,
                                                                            end: 172,
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
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 170,
                                                                        end: 172,
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
                                                                            src (ptr): 0x00007fe08b1c9ee0,
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
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                            ),
                                                                                            start: 166,
                                                                                            end: 167,
                                                                                            as_str(): "%",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                            ),
                                                                                            start: 166,
                                                                                            end: 167,
                                                                                            as_str(): "%",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "modulo",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                        ),
                                                                                        start: 166,
                                                                                        end: 167,
                                                                                        as_str(): "%",
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
                                                                                            src (ptr): 0x00007fe08b1c9ee0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 2008,
                                                                                            end: 2012,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                    ),
                                                                                                    start: 95,
                                                                                                    end: 100,
                                                                                                    as_str(): "index",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                ),
                                                                                                start: 160,
                                                                                                end: 165,
                                                                                                as_str(): "index",
                                                                                            },
                                                                                            mutability: Mutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                            ),
                                                                                            start: 160,
                                                                                            end: 165,
                                                                                            as_str(): "index",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b1c9ee0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 2014,
                                                                                            end: 2019,
                                                                                            as_str(): "other",
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
                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                            ),
                                                                                            start: 168,
                                                                                            end: 169,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                13315,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe08b1c9ee0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 1998,
                                                                                    end: 2137,
                                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: Some(
                                                                                TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                        ),
                                                                                        start: 166,
                                                                                        end: 167,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 160,
                                                                            end: 169,
                                                                            as_str(): "index % 2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b1c9ee0,
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
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 173,
                                                                            end: 174,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13316,
                                                                Span {
                                                                    src (ptr): 0x00007fe08b1c9ee0,
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
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 170,
                                                                        end: 172,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 160,
                                                            end: 174,
                                                            as_str(): "index % 2 == 0",
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
                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                ),
                                                                                                start: 189,
                                                                                                end: 192,
                                                                                                as_str(): "sum",
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
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 199,
                                                                                                                end: 200,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 199,
                                                                                                                end: 200,
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
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 199,
                                                                                                            end: 200,
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
                                                                                                                src (ptr): 0x00007fe08b1c9ee0,
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
                                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 118,
                                                                                                                        end: 121,
                                                                                                                        as_str(): "sum",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 195,
                                                                                                                    end: 198,
                                                                                                                    as_str(): "sum",
                                                                                                                },
                                                                                                                mutability: Mutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                21,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 195,
                                                                                                                end: 198,
                                                                                                                as_str(): "sum",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08b1c9ee0,
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
                                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 95,
                                                                                                                        end: 100,
                                                                                                                        as_str(): "index",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 201,
                                                                                                                    end: 206,
                                                                                                                    as_str(): "index",
                                                                                                                },
                                                                                                                mutability: Mutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                21,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 201,
                                                                                                                end: 206,
                                                                                                                as_str(): "index",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                function_decl_id: DeclId(
                                                                                                    13317,
                                                                                                    Span {
                                                                                                        src (ptr): 0x00007fe08b1c9ee0,
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
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 199,
                                                                                                            end: 200,
                                                                                                            as_str(): "+",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                ),
                                                                                                start: 195,
                                                                                                end: 206,
                                                                                                as_str(): "sum + index",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    31652,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 189,
                                                                                    end: 206,
                                                                                    as_str(): "sum = sum + index",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 189,
                                                                            end: 206,
                                                                            as_str(): "sum = sum + index",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31654,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 217,
                                                            as_str(): "{\n            sum = sum + index;\n        }",
                                                        },
                                                    },
                                                    else: None,
                                                },
                                                return_type: TypeId(
                                                    31657,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 217,
                                                    as_str(): "if index % 2 == 0 {\n            sum = sum + index;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 157,
                                            end: 217,
                                            as_str(): "if index % 2 == 0 {\n            sum = sum + index;\n        }",
                                        },
                                    },
                                    TyAstNode {
                                        content: Expression(
                                            TyExpression {
                                                expression: Reassignment(
                                                    TyReassignment {
                                                        lhs_base_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 227,
                                                                end: 232,
                                                                as_str(): "index",
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
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 241,
                                                                                end: 242,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 241,
                                                                                end: 242,
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
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 241,
                                                                            end: 242,
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
                                                                                src (ptr): 0x00007fe08b1c9ee0,
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
                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                        ),
                                                                                        start: 95,
                                                                                        end: 100,
                                                                                        as_str(): "index",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 235,
                                                                                    end: 240,
                                                                                    as_str(): "index",
                                                                                },
                                                                                mutability: Mutable,
                                                                            },
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 235,
                                                                                end: 240,
                                                                                as_str(): "index",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b1c9ee0,
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
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 243,
                                                                                end: 244,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                function_decl_id: DeclId(
                                                                    13318,
                                                                    Span {
                                                                        src (ptr): 0x00007fe08b1c9ee0,
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
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 241,
                                                                            end: 242,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 235,
                                                                end: 244,
                                                                as_str(): "index + 1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    31664,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 227,
                                                    end: 244,
                                                    as_str(): "index = index + 1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 227,
                                            end: 244,
                                            as_str(): "index = index + 1",
                                        },
                                    },
                                ],
                            },
                        },
                        return_type: TypeId(
                            31666,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 131,
                            end: 251,
                            as_str(): "while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 131,
                    end: 251,
                    as_str(): "while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }",
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
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 260,
                                            end: 261,
                                            as_str(): "+",
                                        },
                                        is_raw_ident: false,
                                    },
                                    BaseIdent {
                                        name_override_opt: Some(
                                            "ops",
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 260,
                                            end: 261,
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
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 260,
                                        end: 261,
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
                                            src (ptr): 0x00007fe08b1c9ee0,
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
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 118,
                                                    end: 121,
                                                    as_str(): "sum",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 256,
                                                end: 259,
                                                as_str(): "sum",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 256,
                                            end: 259,
                                            as_str(): "sum",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08b1c9ee0,
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
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 55,
                                                    end: 59,
                                                    as_str(): "init",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 262,
                                                end: 266,
                                                as_str(): "init",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 262,
                                            end: 266,
                                            as_str(): "init",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13319,
                                Span {
                                    src (ptr): 0x00007fe08b1c9ee0,
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
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 260,
                                        end: 261,
                                        as_str(): "+",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 256,
                            end: 266,
                            as_str(): "sum + init",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 256,
                    end: 266,
                    as_str(): "sum + init",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 55,
                    end: 59,
                    as_str(): "init",
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
                src (ptr): 0x00007fe07c8bc640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
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
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 66,
                    end: 67,
                    as_str(): "n",
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
                src (ptr): 0x00007fe07c8bc640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                ),
                start: 69,
                end: 72,
                as_str(): "u64",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe07c8bc640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
        ),
        start: 48,
        end: 269,
        as_str(): "fn foo(init: u64, n: u64) -> u64 {\n    let mut index = 0;\n    let mut sum = 0;\n    while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }\n    sum + init \n}",
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
        src (ptr): 0x00007fe07c8bc640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
        ),
        start: 77,
        end: 80,
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
            src (ptr): 0x00007fe07c8bc640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
            ),
            start: 274,
            end: 278,
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
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 299,
                                    end: 300,
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
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 303,
                                                end: 306,
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
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 55,
                                                    end: 59,
                                                    as_str(): "init",
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
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 307,
                                                    end: 309,
                                                    as_str(): "11",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 66,
                                                    end: 67,
                                                    as_str(): "n",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 311,
                                                    end: 312,
                                                    as_str(): "4",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13322,
                                        Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 269,
                                            as_str(): "fn foo(init: u64, n: u64) -> u64 {\n    let mut index = 0;\n    let mut sum = 0;\n    while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }\n    sum + init \n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 303,
                                                end: 306,
                                                as_str(): "foo",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 303,
                                    end: 313,
                                    as_str(): "foo(11, 4)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31670,
                            ),
                            type_ascription: TypeId(
                                31670,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 295,
                    end: 314,
                    as_str(): "let x = foo(11, 4);",
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
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 319,
                                        end: 325,
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
                                            src (ptr): 0x00007fe07894a110,
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
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 328,
                                                            end: 330,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 328,
                                                            end: 330,
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
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 328,
                                                        end: 330,
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
                                                            src (ptr): 0x00007fe08b1c9ee0,
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
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 299,
                                                                    end: 300,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 326,
                                                                end: 327,
                                                                as_str(): "x",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31670,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 326,
                                                            end: 327,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1c9ee0,
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
                                                                13,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 331,
                                                            end: 333,
                                                            as_str(): "13",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fe08b1c9ee0,
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
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 328,
                                                        end: 330,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 326,
                                            end: 333,
                                            as_str(): "x == 13",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fe07894a110,
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
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 319,
                                        end: 325,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31680,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 319,
                            end: 334,
                            as_str(): "assert(x == 13)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 319,
                    end: 334,
                    as_str(): "assert(x == 13)",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
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
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 340,
                            end: 344,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe07c8bc640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                    ),
                    start: 340,
                    end: 344,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe07c8bc640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
        ),
        start: 271,
        end: 346,
        as_str(): "fn main() -> bool {\n    let x = foo(11, 4);\n    assert(x == 13);\n    true\n}",
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
        src (ptr): 0x00007fe07c8bc640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
        ),
        start: 284,
        end: 288,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

