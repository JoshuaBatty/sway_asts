
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe082cf6db0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
            ),
            start: 38,
            end: 42,
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
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 63,
                                    end: 75,
                                    as_str(): "uint64_test1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        100000000000,
                                    ),
                                ),
                                return_type: TypeId(
                                    31631,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 83,
                                    end: 95,
                                    as_str(): "100000000000",
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
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 77,
                                    end: 80,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 59,
                    end: 96,
                    as_str(): "let uint64_test1: u64 = 100000000000;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 117,
                                    as_str(): "uint32_test1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U32(
                                        1000000000,
                                    ),
                                ),
                                return_type: TypeId(
                                    31632,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 125,
                                    end: 135,
                                    as_str(): "1000000000",
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
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 119,
                                    end: 122,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 101,
                    end: 136,
                    as_str(): "let uint32_test1: u32 = 1000000000;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 145,
                                    end: 157,
                                    as_str(): "uint16_test1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U16(
                                        10000,
                                    ),
                                ),
                                return_type: TypeId(
                                    31633,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 165,
                                    end: 170,
                                    as_str(): "10000",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                42,
                            ),
                            type_ascription: TypeId(
                                42,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 159,
                                    end: 162,
                                    as_str(): "u16",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 141,
                    end: 171,
                    as_str(): "let uint16_test1: u16 = 10000;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 180,
                                    end: 191,
                                    as_str(): "uint8_test1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U8(
                                        100,
                                    ),
                                ),
                                return_type: TypeId(
                                    31634,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 198,
                                    end: 201,
                                    as_str(): "100",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                50,
                            ),
                            type_ascription: TypeId(
                                50,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 193,
                                    end: 195,
                                    as_str(): "u8",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 176,
                    end: 202,
                    as_str(): "let uint8_test1: u8 = 100;",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 252,
                                        end: 258,
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
                                            src (ptr): 0x00007fe08be33190,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 281,
                                                            end: 283,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 281,
                                                            end: 283,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 281,
                                                        end: 283,
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
                                                            src (ptr): 0x00007fe09e8b2e70,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 272,
                                                                            end: 273,
                                                                            as_str(): "%",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 272,
                                                                            end: 273,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 272,
                                                                        end: 273,
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
                                                                            src (ptr): 0x00007fe09e8b2e70,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 63,
                                                                                    end: 75,
                                                                                    as_str(): "uint64_test1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 259,
                                                                                end: 271,
                                                                                as_str(): "uint64_test1",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 271,
                                                                            as_str(): "uint64_test1",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09e8b2e70,
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
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 274,
                                                                            end: 280,
                                                                            as_str(): "100u64",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13315,
                                                                Span {
                                                                    src (ptr): 0x00007fe09e8b2e70,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 272,
                                                                        end: 273,
                                                                        as_str(): "%",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 280,
                                                            as_str(): "uint64_test1 % 100u64",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09e8b2e70,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 284,
                                                            end: 285,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13316,
                                                Span {
                                                    src (ptr): 0x00007fe09e8b2e70,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 281,
                                                        end: 283,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 259,
                                            end: 285,
                                            as_str(): "uint64_test1 % 100u64 == 0",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007fe08be33190,
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 252,
                                        end: 258,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31642,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 252,
                            end: 286,
                            as_str(): "assert(uint64_test1 % 100u64 == 0)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 252,
                    end: 286,
                    as_str(): "assert(uint64_test1 % 100u64 == 0)",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 292,
                                        end: 298,
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
                                            src (ptr): 0x00007fe08be33190,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 321,
                                                            end: 323,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 321,
                                                            end: 323,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 321,
                                                        end: 323,
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
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3114,
                                                            end: 3118,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 312,
                                                                            end: 313,
                                                                            as_str(): "%",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 312,
                                                                            end: 313,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 312,
                                                                        end: 313,
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
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2174,
                                                                            end: 2178,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 105,
                                                                                    end: 117,
                                                                                    as_str(): "uint32_test1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 299,
                                                                                end: 311,
                                                                                as_str(): "uint32_test1",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 299,
                                                                            end: 311,
                                                                            as_str(): "uint32_test1",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2180,
                                                                            end: 2185,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U32(
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 314,
                                                                            end: 320,
                                                                            as_str(): "100u32",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13319,
                                                                Span {
                                                                    src (ptr): 0x00007fe09e8b2e70,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2164,
                                                                    end: 2303,
                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u32\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 312,
                                                                        end: 313,
                                                                        as_str(): "%",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 299,
                                                            end: 320,
                                                            as_str(): "uint32_test1 % 100u32",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3120,
                                                            end: 3125,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 324,
                                                            end: 325,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13320,
                                                Span {
                                                    src (ptr): 0x00007fe09e8b2e70,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3108,
                                                    end: 3174,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 321,
                                                        end: 323,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 299,
                                            end: 325,
                                            as_str(): "uint32_test1 % 100u32 == 0",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13321,
                                Span {
                                    src (ptr): 0x00007fe08be33190,
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 292,
                                        end: 298,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31650,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 292,
                            end: 326,
                            as_str(): "assert(uint32_test1 % 100u32 == 0)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 292,
                    end: 326,
                    as_str(): "assert(uint32_test1 % 100u32 == 0)",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 332,
                                        end: 338,
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
                                            src (ptr): 0x00007fe08be33190,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 361,
                                                            end: 363,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 361,
                                                            end: 363,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 361,
                                                        end: 363,
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
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3206,
                                                            end: 3210,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 352,
                                                                            end: 353,
                                                                            as_str(): "%",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 352,
                                                                            end: 353,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 352,
                                                                        end: 353,
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
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2340,
                                                                            end: 2344,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 145,
                                                                                    end: 157,
                                                                                    as_str(): "uint16_test1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 339,
                                                                                end: 351,
                                                                                as_str(): "uint16_test1",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 339,
                                                                            end: 351,
                                                                            as_str(): "uint16_test1",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2346,
                                                                            end: 2351,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U16(
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 354,
                                                                            end: 360,
                                                                            as_str(): "100u16",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13323,
                                                                Span {
                                                                    src (ptr): 0x00007fe09e8b2e70,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2330,
                                                                    end: 2469,
                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u16\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 352,
                                                                        end: 353,
                                                                        as_str(): "%",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 339,
                                                            end: 360,
                                                            as_str(): "uint16_test1 % 100u16",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3212,
                                                            end: 3217,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 364,
                                                            end: 365,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fe09e8b2e70,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3200,
                                                    end: 3266,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 361,
                                                        end: 363,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 339,
                                            end: 365,
                                            as_str(): "uint16_test1 % 100u16 == 0",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fe08be33190,
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 332,
                                        end: 338,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31658,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 332,
                            end: 366,
                            as_str(): "assert(uint16_test1 % 100u16 == 0)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 332,
                    end: 366,
                    as_str(): "assert(uint16_test1 % 100u16 == 0)",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 372,
                                        end: 378,
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
                                            src (ptr): 0x00007fe08be33190,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 399,
                                                            end: 401,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 399,
                                                            end: 401,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 399,
                                                        end: 401,
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
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3297,
                                                            end: 3301,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 391,
                                                                            end: 392,
                                                                            as_str(): "%",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 391,
                                                                            end: 392,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 391,
                                                                        end: 392,
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
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2505,
                                                                            end: 2509,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 180,
                                                                                    end: 191,
                                                                                    as_str(): "uint8_test1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 379,
                                                                                end: 390,
                                                                                as_str(): "uint8_test1",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 379,
                                                                            end: 390,
                                                                            as_str(): "uint8_test1",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2511,
                                                                            end: 2516,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U8(
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 393,
                                                                            end: 398,
                                                                            as_str(): "100u8",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13327,
                                                                Span {
                                                                    src (ptr): 0x00007fe09e8b2e70,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2495,
                                                                    end: 2633,
                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u8\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 391,
                                                                        end: 392,
                                                                        as_str(): "%",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 379,
                                                            end: 398,
                                                            as_str(): "uint8_test1 % 100u8",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3303,
                                                            end: 3308,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 402,
                                                            end: 403,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13328,
                                                Span {
                                                    src (ptr): 0x00007fe09e8b2e70,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3291,
                                                    end: 3357,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 399,
                                                        end: 401,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 379,
                                            end: 403,
                                            as_str(): "uint8_test1 % 100u8 == 0",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13329,
                                Span {
                                    src (ptr): 0x00007fe08be33190,
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 372,
                                        end: 378,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31666,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 372,
                            end: 404,
                            as_str(): "assert(uint8_test1 % 100u8 == 0)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 372,
                    end: 404,
                    as_str(): "assert(uint8_test1 % 100u8 == 0)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 415,
                                    end: 427,
                                    as_str(): "uint64_test2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        100000000005,
                                    ),
                                ),
                                return_type: TypeId(
                                    31667,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 435,
                                    end: 447,
                                    as_str(): "100000000005",
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
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 429,
                                    end: 432,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 411,
                    end: 448,
                    as_str(): "let uint64_test2: u64 = 100000000005;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 457,
                                    end: 469,
                                    as_str(): "uint32_test2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U32(
                                        1000000005,
                                    ),
                                ),
                                return_type: TypeId(
                                    31668,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 477,
                                    end: 487,
                                    as_str(): "1000000005",
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
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 471,
                                    end: 474,
                                    as_str(): "u32",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 453,
                    end: 488,
                    as_str(): "let uint32_test2: u32 = 1000000005;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 497,
                                    end: 509,
                                    as_str(): "uint16_test2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U16(
                                        10005,
                                    ),
                                ),
                                return_type: TypeId(
                                    31669,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 517,
                                    end: 522,
                                    as_str(): "10005",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                42,
                            ),
                            type_ascription: TypeId(
                                42,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 511,
                                    end: 514,
                                    as_str(): "u16",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 493,
                    end: 523,
                    as_str(): "let uint16_test2: u16 = 10005;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 532,
                                    end: 543,
                                    as_str(): "uint8_test2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U8(
                                        105,
                                    ),
                                ),
                                return_type: TypeId(
                                    31670,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 550,
                                    end: 553,
                                    as_str(): "105",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                50,
                            ),
                            type_ascription: TypeId(
                                50,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 545,
                                    end: 547,
                                    as_str(): "u8",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 528,
                    end: 554,
                    as_str(): "let uint8_test2: u8 = 105;",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 611,
                                        end: 617,
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
                                            src (ptr): 0x00007fe08be33190,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 640,
                                                            end: 642,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 640,
                                                            end: 642,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 640,
                                                        end: 642,
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
                                                            src (ptr): 0x00007fe09e8b2e70,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 631,
                                                                            end: 632,
                                                                            as_str(): "%",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 631,
                                                                            end: 632,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 631,
                                                                        end: 632,
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
                                                                            src (ptr): 0x00007fe09e8b2e70,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 415,
                                                                                    end: 427,
                                                                                    as_str(): "uint64_test2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 618,
                                                                                end: 630,
                                                                                as_str(): "uint64_test2",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 618,
                                                                            end: 630,
                                                                            as_str(): "uint64_test2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09e8b2e70,
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
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 633,
                                                                            end: 639,
                                                                            as_str(): "100u64",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13331,
                                                                Span {
                                                                    src (ptr): 0x00007fe09e8b2e70,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 631,
                                                                        end: 632,
                                                                        as_str(): "%",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 618,
                                                            end: 639,
                                                            as_str(): "uint64_test2 % 100u64",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09e8b2e70,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 643,
                                                            end: 644,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13332,
                                                Span {
                                                    src (ptr): 0x00007fe09e8b2e70,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 640,
                                                        end: 642,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 618,
                                            end: 644,
                                            as_str(): "uint64_test2 % 100u64 == 5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007fe08be33190,
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 611,
                                        end: 617,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31678,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 611,
                            end: 645,
                            as_str(): "assert(uint64_test2 % 100u64 == 5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 611,
                    end: 645,
                    as_str(): "assert(uint64_test2 % 100u64 == 5)",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 651,
                                        end: 657,
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
                                            src (ptr): 0x00007fe08be33190,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 680,
                                                            end: 682,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 680,
                                                            end: 682,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 680,
                                                        end: 682,
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
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3114,
                                                            end: 3118,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 671,
                                                                            end: 672,
                                                                            as_str(): "%",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 671,
                                                                            end: 672,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 671,
                                                                        end: 672,
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
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2174,
                                                                            end: 2178,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 457,
                                                                                    end: 469,
                                                                                    as_str(): "uint32_test2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 658,
                                                                                end: 670,
                                                                                as_str(): "uint32_test2",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 658,
                                                                            end: 670,
                                                                            as_str(): "uint32_test2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2180,
                                                                            end: 2185,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U32(
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            33,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 673,
                                                                            end: 679,
                                                                            as_str(): "100u32",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13335,
                                                                Span {
                                                                    src (ptr): 0x00007fe09e8b2e70,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2164,
                                                                    end: 2303,
                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u32\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 671,
                                                                        end: 672,
                                                                        as_str(): "%",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 658,
                                                            end: 679,
                                                            as_str(): "uint32_test2 % 100u32",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3120,
                                                            end: 3125,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 683,
                                                            end: 684,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13336,
                                                Span {
                                                    src (ptr): 0x00007fe09e8b2e70,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3108,
                                                    end: 3174,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 680,
                                                        end: 682,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 658,
                                            end: 684,
                                            as_str(): "uint32_test2 % 100u32 == 5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fe08be33190,
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 651,
                                        end: 657,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31686,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 651,
                            end: 685,
                            as_str(): "assert(uint32_test2 % 100u32 == 5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 651,
                    end: 685,
                    as_str(): "assert(uint32_test2 % 100u32 == 5)",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 691,
                                        end: 697,
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
                                            src (ptr): 0x00007fe08be33190,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 720,
                                                            end: 722,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 720,
                                                            end: 722,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 720,
                                                        end: 722,
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
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3206,
                                                            end: 3210,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 711,
                                                                            end: 712,
                                                                            as_str(): "%",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 711,
                                                                            end: 712,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 711,
                                                                        end: 712,
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
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2340,
                                                                            end: 2344,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 497,
                                                                                    end: 509,
                                                                                    as_str(): "uint16_test2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 698,
                                                                                end: 710,
                                                                                as_str(): "uint16_test2",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 698,
                                                                            end: 710,
                                                                            as_str(): "uint16_test2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2346,
                                                                            end: 2351,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U16(
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            42,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 713,
                                                                            end: 719,
                                                                            as_str(): "100u16",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13339,
                                                                Span {
                                                                    src (ptr): 0x00007fe09e8b2e70,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2330,
                                                                    end: 2469,
                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u16\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 711,
                                                                        end: 712,
                                                                        as_str(): "%",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            42,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 698,
                                                            end: 719,
                                                            as_str(): "uint16_test2 % 100u16",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3212,
                                                            end: 3217,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 723,
                                                            end: 724,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13340,
                                                Span {
                                                    src (ptr): 0x00007fe09e8b2e70,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3200,
                                                    end: 3266,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 720,
                                                        end: 722,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 698,
                                            end: 724,
                                            as_str(): "uint16_test2 % 100u16 == 5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13341,
                                Span {
                                    src (ptr): 0x00007fe08be33190,
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 691,
                                        end: 697,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31694,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 691,
                            end: 725,
                            as_str(): "assert(uint16_test2 % 100u16 == 5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 691,
                    end: 725,
                    as_str(): "assert(uint16_test2 % 100u16 == 5)",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 731,
                                        end: 737,
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
                                            src (ptr): 0x00007fe08be33190,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 760,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 760,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 758,
                                                        end: 760,
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
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3297,
                                                            end: 3301,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 750,
                                                                            end: 751,
                                                                            as_str(): "%",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 750,
                                                                            end: 751,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 750,
                                                                        end: 751,
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
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2505,
                                                                            end: 2509,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 532,
                                                                                    end: 543,
                                                                                    as_str(): "uint8_test2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 738,
                                                                                end: 749,
                                                                                as_str(): "uint8_test2",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 738,
                                                                            end: 749,
                                                                            as_str(): "uint8_test2",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09e8b2e70,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2511,
                                                                            end: 2516,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U8(
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 752,
                                                                            end: 757,
                                                                            as_str(): "100u8",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13343,
                                                                Span {
                                                                    src (ptr): 0x00007fe09e8b2e70,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 2495,
                                                                    end: 2633,
                                                                    as_str(): "fn modulo(self, other: Self) -> Self {\n        asm(r1: self, r2: other, r3) {\n            mod r3 r1 r2;\n            r3: u8\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 750,
                                                                        end: 751,
                                                                        as_str(): "%",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            50,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 738,
                                                            end: 757,
                                                            as_str(): "uint8_test2 % 100u8",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09e8b2e70,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3303,
                                                            end: 3308,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 761,
                                                            end: 762,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13344,
                                                Span {
                                                    src (ptr): 0x00007fe09e8b2e70,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3291,
                                                    end: 3357,
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
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 758,
                                                        end: 760,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 738,
                                            end: 762,
                                            as_str(): "uint8_test2 % 100u8 == 5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13345,
                                Span {
                                    src (ptr): 0x00007fe08be33190,
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 731,
                                        end: 737,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31702,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 731,
                            end: 763,
                            as_str(): "assert(uint8_test2 % 100u8 == 5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 731,
                    end: 763,
                    as_str(): "assert(uint8_test2 % 100u8 == 5)",
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
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 770,
                            end: 774,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe082cf6db0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                    ),
                    start: 770,
                    end: 774,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe082cf6db0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
        ),
        start: 35,
        end: 776,
        as_str(): "fn main() -> bool {\n    let uint64_test1: u64 = 100000000000;\n    let uint32_test1: u32 = 1000000000;\n    let uint16_test1: u16 = 10000;\n    let uint8_test1: u8 = 100;\n\n    // Ensure 0 remainder returns correctly\n    assert(uint64_test1 % 100u64 == 0);\n    assert(uint32_test1 % 100u32 == 0);\n    assert(uint16_test1 % 100u16 == 0);\n    assert(uint8_test1 % 100u8 == 0);\n\n    let uint64_test2: u64 = 100000000005;\n    let uint32_test2: u32 = 1000000005;\n    let uint16_test2: u16 = 10005;\n    let uint8_test2: u8 = 105;\n\n    // Ensure non zero remainder returns correctly\n    assert(uint64_test2 % 100u64 == 5);\n    assert(uint32_test2 % 100u32 == 5);\n    assert(uint16_test2 % 100u16 == 5);\n    assert(uint8_test2 % 100u8 == 5);\n\n    true\n}",
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
        src (ptr): 0x00007fe082cf6db0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
        ),
        start: 48,
        end: 52,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

