TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 12,
            end: 15,
            as_str(): "log",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IntrinsicFunction(
                            TyIntrinsicFunctionKind {
                                kind: Log,
                                arguments: [
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 24,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 46,
                                                end: 51,
                                                as_str(): "value",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31635,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 46,
                                            end: 51,
                                            as_str(): "value",
                                        },
                                    },
                                ],
                                type_arguments: [],
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 35,
                                    end: 52,
                                    as_str(): "__log::<T>(value)",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31642,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 35,
                            end: 52,
                            as_str(): "__log::<T>(value)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 35,
                    end: 52,
                    as_str(): "__log::<T>(value)",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 19,
                    end: 24,
                    as_str(): "value",
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
                31635,
            ),
            initial_type_id: TypeId(
                31636,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 26,
                end: 27,
                as_str(): "T",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0b4550000,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
        ),
        start: 9,
        end: 55,
        as_str(): "fn log<T>(value: T) {\n    __log::<T>(value);\n}",
    },
    attributes: {},
    return_type: TypeId(
        31638,
    ),
    initial_return_type: TypeId(
        31637,
    ),
    type_parameters: [
        T: TypeId(31635),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0b4550000,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
        ),
        start: 9,
        end: 28,
        as_str(): "fn log<T>(value: T)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 64,
            end: 74,
            as_str(): "TestStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 84,
                    end: 91,
                    as_str(): "field_1",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 84,
                end: 97,
                as_str(): "field_1: bool",
            },
            type_span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 93,
                end: 97,
                as_str(): "bool",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 103,
                    end: 110,
                    as_str(): "field_2",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31644,
            ),
            initial_type_id: TypeId(
                31645,
            ),
            span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 103,
                end: 113,
                as_str(): "field_2: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 112,
                end: 113,
                as_str(): "T",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 119,
                    end: 126,
                    as_str(): "field_3",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 119,
                end: 131,
                as_str(): "field_3: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 128,
                end: 131,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(31644),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0b4550000,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
        ),
        start: 57,
        end: 134,
        as_str(): "struct TestStruct<T> {\n    field_1: bool,\n    field_2: T,\n    field_3: u64,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 141,
            end: 149,
            as_str(): "TestEnum",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 156,
                    end: 166,
                    as_str(): "VariantOne",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31647,
            ),
            initial_type_id: TypeId(
                31646,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 168,
                end: 170,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 156,
                end: 170,
                as_str(): "VariantOne: ()",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 176,
                    end: 186,
                    as_str(): "VariantTwo",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31649,
            ),
            initial_type_id: TypeId(
                31648,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 188,
                end: 190,
                as_str(): "()",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 176,
                end: 190,
                as_str(): "VariantTwo: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0b4550000,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
        ),
        start: 136,
        end: 193,
        as_str(): "enum TestEnum {\n    VariantOne: (),\n    VariantTwo: (),\n}",
    },
    visibility: Private,
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 204,
            end: 210,
            as_str(): "Option",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(31650),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 220,
                    end: 224,
                    as_str(): "None",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31652,
            ),
            initial_type_id: TypeId(
                31651,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 226,
                end: 228,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 220,
                end: 228,
                as_str(): "None: ()",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 234,
                    end: 238,
                    as_str(): "Some",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31650,
            ),
            initial_type_id: TypeId(
                31653,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 240,
                end: 241,
                as_str(): "T",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0b4550000,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                ),
                start: 234,
                end: 241,
                as_str(): "Some: T",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0b4550000,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
        ),
        start: 195,
        end: 244,
        as_str(): "pub enum Option<T> {\n    None: (),\n    Some: T,\n}",
    },
    visibility: Public,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 249,
            end: 253,
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
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 274,
                                    end: 275,
                                    as_str(): "k",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            239,
                                            134,
                                            175,
                                            169,
                                            105,
                                            108,
                                            240,
                                            220,
                                            99,
                                            133,
                                            226,
                                            196,
                                            7,
                                            166,
                                            225,
                                            89,
                                            161,
                                            16,
                                            60,
                                            239,
                                            183,
                                            226,
                                            174,
                                            6,
                                            54,
                                            251,
                                            51,
                                            211,
                                            203,
                                            42,
                                            158,
                                            74,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 284,
                                    end: 350,
                                    as_str(): "0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                59,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 277,
                                    end: 281,
                                    as_str(): "b256",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 270,
                    end: 351,
                    as_str(): "let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 360,
                                    end: 361,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    String(
                                        Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 373,
                                            end: 377,
                                            as_str(): "Fuel",
                                        },
                                    ),
                                ),
                                return_type: TypeId(
                                    31656,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 372,
                                    end: 378,
                                    as_str(): "\"Fuel\"",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31656,
                            ),
                            type_ascription: TypeId(
                                31655,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 363,
                                    end: 369,
                                    as_str(): "str[4]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 356,
                    end: 379,
                    as_str(): "let a: str[4] = \"Fuel\";",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 388,
                                    end: 389,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
                                        TyExpression {
                                            expression: Literal(
                                                U8(
                                                    1,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                50,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 402,
                                                end: 405,
                                                as_str(): "1u8",
                                            },
                                        },
                                        TyExpression {
                                            expression: Literal(
                                                U8(
                                                    2,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                50,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 407,
                                                end: 410,
                                                as_str(): "2u8",
                                            },
                                        },
                                        TyExpression {
                                            expression: Literal(
                                                U8(
                                                    3,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                50,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 412,
                                                end: 415,
                                                as_str(): "3u8",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    31663,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 401,
                                    end: 416,
                                    as_str(): "[1u8, 2u8, 3u8]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31663,
                            ),
                            type_ascription: TypeId(
                                31658,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 391,
                                    end: 398,
                                    as_str(): "[u8; 3]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 384,
                    end: 417,
                    as_str(): "let b: [u8; 3] = [1u8, 2u8, 3u8];",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 426,
                                    end: 437,
                                    as_str(): "test_struct",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 64,
                                            end: 74,
                                            as_str(): "TestStruct",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 461,
                                                    end: 468,
                                                    as_str(): "field_1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 470,
                                                    end: 474,
                                                    as_str(): "true",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 484,
                                                    end: 491,
                                                    as_str(): "field_2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 274,
                                                            end: 275,
                                                            as_str(): "k",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 493,
                                                        end: 494,
                                                        as_str(): "k",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    59,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 493,
                                                    end: 494,
                                                    as_str(): "k",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 504,
                                                    end: 511,
                                                    as_str(): "field_3",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        11,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 513,
                                                    end: 515,
                                                    as_str(): "11",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 440,
                                        end: 450,
                                        as_str(): "TestStruct",
                                    },
                                },
                                return_type: TypeId(
                                    31667,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 440,
                                    end: 522,
                                    as_str(): "TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31667,
                            ),
                            type_ascription: TypeId(
                                31664,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 422,
                    end: 523,
                    as_str(): "let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 533,
                                    end: 542,
                                    as_str(): "test_enum",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 141,
                                                end: 149,
                                                as_str(): "TestEnum",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 156,
                                                        end: 166,
                                                        as_str(): "VariantOne",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31647,
                                                ),
                                                initial_type_id: TypeId(
                                                    31646,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 168,
                                                    end: 170,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 156,
                                                    end: 170,
                                                    as_str(): "VariantOne: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 176,
                                                        end: 186,
                                                        as_str(): "VariantTwo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31649,
                                                ),
                                                initial_type_id: TypeId(
                                                    31648,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 190,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 176,
                                                    end: 190,
                                                    as_str(): "VariantTwo: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 136,
                                            end: 193,
                                            as_str(): "enum TestEnum {\n    VariantOne: (),\n    VariantTwo: (),\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 176,
                                            end: 186,
                                            as_str(): "VariantTwo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 1,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 545,
                                        end: 553,
                                        as_str(): "TestEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 555,
                                        end: 565,
                                        as_str(): "VariantTwo",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 555,
                                            end: 565,
                                            as_str(): "VariantTwo",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31673,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 555,
                                    end: 565,
                                    as_str(): "VariantTwo",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31673,
                            ),
                            type_ascription: TypeId(
                                31672,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 529,
                    end: 566,
                    as_str(): "let test_enum = TestEnum::VariantTwo;",
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
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 571,
                                        end: 574,
                                        as_str(): "log",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 19,
                                            end: 24,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 274,
                                                    end: 275,
                                                    as_str(): "k",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 575,
                                                end: 576,
                                                as_str(): "k",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            59,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 575,
                                            end: 576,
                                            as_str(): "k",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13320,
                                Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 55,
                                    as_str(): "fn log<T>(value: T) {\n    __log::<T>(value);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 571,
                                        end: 574,
                                        as_str(): "log",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31677,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 571,
                            end: 577,
                            as_str(): "log(k)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 571,
                    end: 577,
                    as_str(): "log(k)",
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
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 583,
                                        end: 586,
                                        as_str(): "log",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 19,
                                            end: 24,
                                            as_str(): "value",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 587,
                                            end: 589,
                                            as_str(): "42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13322,
                                Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 55,
                                    as_str(): "fn log<T>(value: T) {\n    __log::<T>(value);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 583,
                                        end: 586,
                                        as_str(): "log",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31682,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 583,
                            end: 590,
                            as_str(): "log(42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 583,
                    end: 590,
                    as_str(): "log(42)",
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
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 596,
                                        end: 599,
                                        as_str(): "log",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 19,
                                            end: 24,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U32(
                                                42,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            33,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 600,
                                            end: 605,
                                            as_str(): "42u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13324,
                                Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 55,
                                    as_str(): "fn log<T>(value: T) {\n    __log::<T>(value);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 596,
                                        end: 599,
                                        as_str(): "log",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31686,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 596,
                            end: 606,
                            as_str(): "log(42u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 596,
                    end: 606,
                    as_str(): "log(42u32)",
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
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 612,
                                        end: 615,
                                        as_str(): "log",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 19,
                                            end: 24,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U16(
                                                42,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            42,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 616,
                                            end: 621,
                                            as_str(): "42u16",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13326,
                                Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 55,
                                    as_str(): "fn log<T>(value: T) {\n    __log::<T>(value);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 612,
                                        end: 615,
                                        as_str(): "log",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31690,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 612,
                            end: 622,
                            as_str(): "log(42u16)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 612,
                    end: 622,
                    as_str(): "log(42u16)",
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
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 628,
                                        end: 631,
                                        as_str(): "log",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 19,
                                            end: 24,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U8(
                                                42,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            50,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 632,
                                            end: 636,
                                            as_str(): "42u8",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13328,
                                Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 55,
                                    as_str(): "fn log<T>(value: T) {\n    __log::<T>(value);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 628,
                                        end: 631,
                                        as_str(): "log",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31694,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 628,
                            end: 637,
                            as_str(): "log(42u8)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 628,
                    end: 637,
                    as_str(): "log(42u8)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IntrinsicFunction(
                            TyIntrinsicFunctionKind {
                                kind: Log,
                                arguments: [
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 360,
                                                    end: 361,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 649,
                                                end: 650,
                                                as_str(): "a",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31656,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 649,
                                            end: 650,
                                            as_str(): "a",
                                        },
                                    },
                                ],
                                type_arguments: [],
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 643,
                                    end: 651,
                                    as_str(): "__log(a)",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31698,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 643,
                            end: 651,
                            as_str(): "__log(a)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 643,
                    end: 651,
                    as_str(): "__log(a)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IntrinsicFunction(
                            TyIntrinsicFunctionKind {
                                kind: Log,
                                arguments: [
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 388,
                                                    end: 389,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 663,
                                                end: 664,
                                                as_str(): "b",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31701,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 663,
                                            end: 664,
                                            as_str(): "b",
                                        },
                                    },
                                ],
                                type_arguments: [],
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 657,
                                    end: 665,
                                    as_str(): "__log(b)",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31703,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 657,
                            end: 665,
                            as_str(): "__log(b)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 657,
                    end: 665,
                    as_str(): "__log(b)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IntrinsicFunction(
                            TyIntrinsicFunctionKind {
                                kind: Log,
                                arguments: [
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 426,
                                                    end: 437,
                                                    as_str(): "test_struct",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 677,
                                                end: 688,
                                                as_str(): "test_struct",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31667,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 677,
                                            end: 688,
                                            as_str(): "test_struct",
                                        },
                                    },
                                ],
                                type_arguments: [],
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 671,
                                    end: 689,
                                    as_str(): "__log(test_struct)",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31707,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 671,
                            end: 689,
                            as_str(): "__log(test_struct)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 671,
                    end: 689,
                    as_str(): "__log(test_struct)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IntrinsicFunction(
                            TyIntrinsicFunctionKind {
                                kind: Log,
                                arguments: [
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 533,
                                                    end: 542,
                                                    as_str(): "test_enum",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 701,
                                                end: 710,
                                                as_str(): "test_enum",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31673,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 701,
                                            end: 710,
                                            as_str(): "test_enum",
                                        },
                                    },
                                ],
                                type_arguments: [],
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 695,
                                    end: 711,
                                    as_str(): "__log(test_enum)",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31711,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 695,
                            end: 711,
                            as_str(): "__log(test_enum)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 695,
                    end: 711,
                    as_str(): "__log(test_enum)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IntrinsicFunction(
                            TyIntrinsicFunctionKind {
                                kind: Log,
                                arguments: [
                                    TyExpression {
                                        expression: EnumInstantiation {
                                            enum_decl: TyEnumDeclaration {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0cb02d6c0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                        ),
                                                        start: 2471,
                                                        end: 2477,
                                                        as_str(): "Option",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_parameters: [
                                                    T: TypeId(31714),
                                                ],
                                                attributes: {
                                                    DocComment: [
                                                        Attribute {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "doc-comment",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0cb02d6c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 2386,
                                                                    end: 2461,
                                                                    as_str(): "/// The `Option` type. See [the module level documentation](self) for more.",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            args: [
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0cb02d6c0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                        ),
                                                                        start: 2389,
                                                                        end: 2461,
                                                                        as_str(): " The `Option` type. See [the module level documentation](self) for more.",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0cb02d6c0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                ),
                                                                start: 2386,
                                                                end: 2461,
                                                                as_str(): "/// The `Option` type. See [the module level documentation](self) for more.",
                                                            },
                                                        },
                                                    ],
                                                },
                                                variants: [
                                                    TyEnumVariant {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0cb02d6c0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                ),
                                                                start: 2505,
                                                                end: 2509,
                                                                as_str(): "None",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        type_id: TypeId(
                                                            7589,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7588,
                                                        ),
                                                        type_span: Span {
                                                            src (ptr): 0x00007fe0cb02d6c0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                            ),
                                                            start: 2511,
                                                            end: 2513,
                                                            as_str(): "()",
                                                        },
                                                        tag: 0,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0cb02d6c0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                            ),
                                                            start: 2505,
                                                            end: 2513,
                                                            as_str(): "None: ()",
                                                        },
                                                        attributes: {
                                                            DocComment: [
                                                                Attribute {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "doc-comment",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0cb02d6c0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 2487,
                                                                            end: 2500,
                                                                            as_str(): "/// No value.",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    args: [
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0cb02d6c0,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                ),
                                                                                start: 2490,
                                                                                end: 2500,
                                                                                as_str(): " No value.",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0cb02d6c0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                        ),
                                                                        start: 2487,
                                                                        end: 2500,
                                                                        as_str(): "/// No value.",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    },
                                                    TyEnumVariant {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0cb02d6c0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                ),
                                                                start: 2551,
                                                                end: 2555,
                                                                as_str(): "Some",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        type_id: TypeId(
                                                            31714,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7590,
                                                        ),
                                                        type_span: Span {
                                                            src (ptr): 0x00007fe0cb02d6c0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                            ),
                                                            start: 2557,
                                                            end: 2558,
                                                            as_str(): "T",
                                                        },
                                                        tag: 1,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0cb02d6c0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                            ),
                                                            start: 2551,
                                                            end: 2558,
                                                            as_str(): "Some: T",
                                                        },
                                                        attributes: {
                                                            DocComment: [
                                                                Attribute {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "doc-comment",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0cb02d6c0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 2519,
                                                                            end: 2546,
                                                                            as_str(): "/// Some value of type `T`.",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    args: [
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0cb02d6c0,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                ),
                                                                                start: 2522,
                                                                                end: 2546,
                                                                                as_str(): " Some value of type `T`.",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0cb02d6c0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                        ),
                                                                        start: 2519,
                                                                        end: 2546,
                                                                        as_str(): "/// Some value of type `T`.",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe0cb02d6c0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 2462,
                                                    end: 2561,
                                                    as_str(): "pub enum Option<T> {\n    /// No value.\n    None: (),\n    /// Some value of type `T`.\n    Some: T,\n}",
                                                },
                                                visibility: Public,
                                            },
                                            variant_name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0cb02d6c0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 2551,
                                                    end: 2555,
                                                    as_str(): "Some",
                                                },
                                                is_raw_ident: false,
                                            },
                                            tag: 1,
                                            contents: Some(
                                                TyExpression {
                                                    expression: StructExpression {
                                                        struct_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 74,
                                                                as_str(): "TestStruct",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        fields: [
                                                            TyStructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 757,
                                                                        end: 764,
                                                                        as_str(): "field_1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: TyExpression {
                                                                    expression: Literal(
                                                                        Boolean(
                                                                            true,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 766,
                                                                        end: 770,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                            },
                                                            TyStructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 780,
                                                                        end: 787,
                                                                        as_str(): "field_2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 789,
                                                                        end: 791,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            },
                                                            TyStructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 801,
                                                                        end: 808,
                                                                        as_str(): "field_3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 810,
                                                                        end: 812,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 736,
                                                            end: 746,
                                                            as_str(): "TestStruct",
                                                        },
                                                    },
                                                    return_type: TypeId(
                                                        31760,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 736,
                                                        end: 819,
                                                        as_str(): "TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }",
                                                    },
                                                },
                                            ),
                                            enum_instantiation_span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 723,
                                                end: 729,
                                                as_str(): "Option",
                                            },
                                            variant_instantiation_span: Span {
                                                src (ptr): 0x00007fe0b4550000,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                ),
                                                start: 731,
                                                end: 735,
                                                as_str(): "Some",
                                            },
                                            type_binding: TypeBinding {
                                                inner: (),
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 723,
                                                    end: 735,
                                                    as_str(): "Option::Some",
                                                },
                                            },
                                        },
                                        return_type: TypeId(
                                            31766,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 731,
                                            end: 735,
                                            as_str(): "Some",
                                        },
                                    },
                                ],
                                type_arguments: [],
                                span: Span {
                                    src (ptr): 0x00007fe0b4550000,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                    ),
                                    start: 717,
                                    end: 821,
                                    as_str(): "__log(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }))",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31768,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 717,
                            end: 821,
                            as_str(): "__log(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 717,
                    end: 821,
                    as_str(): "__log(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }))",
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
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 828,
                            end: 832,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0b4550000,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                    ),
                    start: 828,
                    end: 832,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0b4550000,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
        ),
        start: 246,
        end: 834,
        as_str(): "fn main() -> bool {\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    log(k);\n    log(42);\n    log(42u32);\n    log(42u16);\n    log(42u8);\n    __log(a);\n    __log(b);\n    __log(test_struct);\n    __log(test_enum);\n    __log(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }));\n\n    true\n}",
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
        src (ptr): 0x00007fe0b4550000,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
        ),
        start: 259,
        end: 263,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

