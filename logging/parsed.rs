[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 35,
                                                        end: 40,
                                                        as_str(): "__log",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Log,
                                                    type_arguments: [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                31628,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                31628,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 43,
                                                                end: 44,
                                                                as_str(): "T",
                                                            },
                                                        },
                                                    ],
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
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 46,
                                                                    end: 51,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 29,
                            end: 55,
                            as_str(): "{\n    __log::<T>(value);\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
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
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 27,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                    span: Span {
                        src (ptr): 0x00007fe0b4550000,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                        ),
                        start: 9,
                        end: 55,
                        as_str(): "fn log<T>(value: T) {\n    __log::<T>(value);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [
                        T: TypeId(31629),
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 9,
            end: 55,
            as_str(): "fn log<T>(value: T) {\n    __log::<T>(value);\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
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
                    attributes: {},
                    fields: [
                        StructField {
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
                            attributes: {},
                            type_info: Boolean,
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
                        },
                        StructField {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 112,
                                        end: 113,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                        },
                        StructField {
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
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
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
                        },
                    ],
                    type_parameters: [
                        T: TypeId(31630),
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 57,
            end: 134,
            as_str(): "struct TestStruct<T> {\n    field_1: bool,\n    field_2: T,\n    field_3: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
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
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
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
                            attributes: {},
                            type_info: Tuple(
                                [],
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
                        },
                        EnumVariant {
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
                            attributes: {},
                            type_info: Tuple(
                                [],
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
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 136,
            end: 193,
            as_str(): "enum TestEnum {\n    VariantOne: (),\n    VariantTwo: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
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
                    attributes: {},
                    type_parameters: [
                        T: TypeId(31631),
                    ],
                    variants: [
                        EnumVariant {
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
                            attributes: {},
                            type_info: Tuple(
                                [],
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
                        },
                        EnumVariant {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 240,
                                        end: 241,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 195,
            end: 244,
            as_str(): "pub enum Option<T> {\n    None: (),\n    Some: T,\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: B256,
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
                                            body: Expression {
                                                kind: Literal(
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Str(
                                                Length {
                                                    val: 4,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 367,
                                                        end: 368,
                                                        as_str(): "4",
                                                    },
                                                },
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
                                            body: Expression {
                                                kind: Literal(
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        50,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        50,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 392,
                                                        end: 394,
                                                        as_str(): "u8",
                                                    },
                                                },
                                                Length {
                                                    val: 3,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 396,
                                                        end: 397,
                                                        as_str(): "3",
                                                    },
                                                },
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
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        1,
                                                                    ),
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
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        2,
                                                                    ),
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
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        3,
                                                                    ),
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
                                                        length_span: None,
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 440,
                                                                        end: 450,
                                                                        as_str(): "TestStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
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
                                                        fields: [
                                                            StructExpressionField {
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
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Boolean(
                                                                            true,
                                                                        ),
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
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 461,
                                                                    end: 474,
                                                                    as_str(): "field_1: true",
                                                                },
                                                            },
                                                            StructExpressionField {
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
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 493,
                                                                                end: 494,
                                                                                as_str(): "k",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
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
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 484,
                                                                    end: 494,
                                                                    as_str(): "field_2: k",
                                                                },
                                                            },
                                                            StructExpressionField {
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
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            11,
                                                                        ),
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
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 504,
                                                                    end: 515,
                                                                    as_str(): "field_3: 11",
                                                                },
                                                            },
                                                        ],
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: DelineatedPath(
                                                    DelineatedPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b4550000,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                            ),
                                                                            start: 545,
                                                                            end: 553,
                                                                            as_str(): "TestEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 555,
                                                                        end: 565,
                                                                        as_str(): "VariantTwo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
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
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 545,
                                                    end: 565,
                                                    as_str(): "TestEnum::VariantTwo",
                                                },
                                            },
                                            is_mutable: false,
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
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
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 575,
                                                                    end: 576,
                                                                    as_str(): "k",
                                                                },
                                                                is_raw_ident: false,
                                                            },
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
                                                ],
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
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
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                42,
                                                            ),
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
                                                ],
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
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
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            U32(
                                                                42,
                                                            ),
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
                                                ],
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
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
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            U16(
                                                                42,
                                                            ),
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
                                                ],
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
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
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            U8(
                                                                42,
                                                            ),
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
                                                ],
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 643,
                                                        end: 648,
                                                        as_str(): "__log",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Log,
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
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 649,
                                                                    end: 650,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 657,
                                                        end: 662,
                                                        as_str(): "__log",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Log,
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
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 663,
                                                                    end: 664,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 671,
                                                        end: 676,
                                                        as_str(): "__log",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Log,
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
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 677,
                                                                    end: 688,
                                                                    as_str(): "test_struct",
                                                                },
                                                                is_raw_ident: false,
                                                            },
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 695,
                                                        end: 700,
                                                        as_str(): "__log",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Log,
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
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 701,
                                                                    end: 710,
                                                                    as_str(): "test_enum",
                                                                },
                                                                is_raw_ident: false,
                                                            },
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 717,
                                                        end: 722,
                                                        as_str(): "__log",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Log,
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
                                                arguments: [
                                                    Expression {
                                                        kind: AmbiguousPathExpression(
                                                            AmbiguousPathExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: AmbiguousSuffix {
                                                                            before: TypeBinding {
                                                                                inner: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 723,
                                                                                        end: 729,
                                                                                        as_str(): "Option",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 723,
                                                                                    end: 729,
                                                                                    as_str(): "Option",
                                                                                },
                                                                            },
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 731,
                                                                                    end: 735,
                                                                                    as_str(): "Some",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        is_absolute: false,
                                                                    },
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
                                                                args: [
                                                                    Expression {
                                                                        kind: Struct(
                                                                            StructExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                ),
                                                                                                start: 736,
                                                                                                end: 746,
                                                                                                as_str(): "TestStruct",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
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
                                                                                fields: [
                                                                                    StructExpressionField {
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
                                                                                        value: Expression {
                                                                                            kind: Literal(
                                                                                                Boolean(
                                                                                                    true,
                                                                                                ),
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
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 757,
                                                                                            end: 770,
                                                                                            as_str(): "field_1: true",
                                                                                        },
                                                                                    },
                                                                                    StructExpressionField {
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
                                                                                        value: Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    42,
                                                                                                ),
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
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 780,
                                                                                            end: 791,
                                                                                            as_str(): "field_2: 42",
                                                                                        },
                                                                                    },
                                                                                    StructExpressionField {
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
                                                                                        value: Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    42,
                                                                                                ),
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
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 801,
                                                                                            end: 812,
                                                                                            as_str(): "field_3: 42",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
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
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 723,
                                                            end: 820,
                                                            as_str(): "Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    })",
                                                        },
                                                    },
                                                ],
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 264,
                            end: 834,
                            as_str(): "{\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    log(k);\n    log(42);\n    log(42u32);\n    log(42u16);\n    log(42u8);\n    __log(a);\n    __log(b);\n    __log(test_struct);\n    __log(test_enum);\n    __log(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }));\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0b4550000,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                        ),
                        start: 246,
                        end: 834,
                        as_str(): "fn main() -> bool {\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    log(k);\n    log(42);\n    log(42u32);\n    log(42u16);\n    log(42u8);\n    __log(a);\n    __log(b);\n    __log(test_struct);\n    __log(test_enum);\n    __log(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }));\n\n    true\n}",
                    },
                    return_type: Boolean,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0b4550000,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
            ),
            start: 246,
            end: 834,
            as_str(): "fn main() -> bool {\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    log(k);\n    log(42);\n    log(42u32);\n    log(42u16);\n    log(42u8);\n    __log(a);\n    __log(b);\n    __log(test_struct);\n    __log(test_enum);\n    __log(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }));\n\n    true\n}",
        },
    },
]
