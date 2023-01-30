TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb1359303f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
            ),
            start: 12,
            end: 16,
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
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 48,
                                    end: 49,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        15,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 52,
                                    end: 55,
                                    as_str(): "0xF",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                4,
                            ),
                            type_ascription: TypeId(
                                4,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 44,
                    end: 56,
                    as_str(): "let x = 0xF;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 66,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        255,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 69,
                                    end: 73,
                                    as_str(): "0xFF",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                6,
                            ),
                            type_ascription: TypeId(
                                6,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 61,
                    end: 74,
                    as_str(): "let x = 0xFF;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 83,
                                    end: 84,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        4095,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 87,
                                    end: 92,
                                    as_str(): "0xFFF",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7,
                            ),
                            type_ascription: TypeId(
                                7,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 79,
                    end: 93,
                    as_str(): "let x = 0xFFF;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 102,
                                    end: 103,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        43690,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 106,
                                    end: 112,
                                    as_str(): "0xAAAA",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                8,
                            ),
                            type_ascription: TypeId(
                                8,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 98,
                    end: 113,
                    as_str(): "let x = 0xAAAA;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 122,
                                    end: 123,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        734003,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 126,
                                    end: 134,
                                    as_str(): "0xB_3333",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                9,
                            ),
                            type_ascription: TypeId(
                                9,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 118,
                    end: 135,
                    as_str(): "let x = 0xB_3333;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 144,
                                    end: 145,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        4294919236,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 148,
                                    end: 159,
                                    as_str(): "0xFFFF_4444",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                10,
                            ),
                            type_ascription: TypeId(
                                10,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 140,
                    end: 160,
                    as_str(): "let x = 0xFFFF_4444;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 169,
                                    end: 170,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        18764998447377,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 173,
                                    end: 189,
                                    as_str(): "0x1111_1111_1111",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                11,
                            ),
                            type_ascription: TypeId(
                                11,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 165,
                    end: 190,
                    as_str(): "let x = 0x1111_1111_1111;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 199,
                                    end: 200,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        18446744073709551615,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 203,
                                    end: 222,
                                    as_str(): "0xFFFF_FFFFFFFFFFFF",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                12,
                            ),
                            type_ascription: TypeId(
                                12,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 195,
                    end: 223,
                    as_str(): "let x = 0xFFFF_FFFFFFFFFFFF;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 232,
                                    end: 233,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    14,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 236,
                                    end: 317,
                                    as_str(): "0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                14,
                            ),
                            type_ascription: TypeId(
                                13,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 228,
                    end: 318,
                    as_str(): "let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 328,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    14,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 331,
                                    end: 412,
                                    as_str(): "0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                14,
                            ),
                            type_ascription: TypeId(
                                15,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 323,
                    end: 413,
                    as_str(): "let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 437,
                                    end: 438,
                                    as_str(): "y",
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
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 441,
                                    end: 444,
                                    as_str(): "0b1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                16,
                            ),
                            type_ascription: TypeId(
                                16,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 433,
                    end: 445,
                    as_str(): "let y = 0b1;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 454,
                                    end: 455,
                                    as_str(): "y",
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
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 458,
                                    end: 462,
                                    as_str(): "0b11",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                17,
                            ),
                            type_ascription: TypeId(
                                17,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 450,
                    end: 463,
                    as_str(): "let y = 0b11;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 472,
                                    end: 473,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        7,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 476,
                                    end: 481,
                                    as_str(): "0b111",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                18,
                            ),
                            type_ascription: TypeId(
                                18,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 468,
                    end: 482,
                    as_str(): "let y = 0b111;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 491,
                                    end: 492,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        15,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 495,
                                    end: 501,
                                    as_str(): "0b1111",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                19,
                            ),
                            type_ascription: TypeId(
                                19,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 487,
                    end: 502,
                    as_str(): "let y = 0b1111;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 511,
                                    end: 512,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        31,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 515,
                                    end: 523,
                                    as_str(): "0b1_1111",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                20,
                            ),
                            type_ascription: TypeId(
                                20,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 507,
                    end: 524,
                    as_str(): "let y = 0b1_1111;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 533,
                                    end: 534,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        240,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 537,
                                    end: 548,
                                    as_str(): "0b1111_0000",
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
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 529,
                    end: 549,
                    as_str(): "let y = 0b1111_0000;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 558,
                                    end: 559,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        4095,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 562,
                                    end: 578,
                                    as_str(): "0b1111_1111_1111",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                22,
                            ),
                            type_ascription: TypeId(
                                22,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 554,
                    end: 579,
                    as_str(): "let y = 0b1111_1111_1111;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 588,
                                    end: 589,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        65535,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 592,
                                    end: 611,
                                    as_str(): "0b1111_111111111111",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                23,
                            ),
                            type_ascription: TypeId(
                                23,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 584,
                    end: 612,
                    as_str(): "let y = 0b1111_111111111111;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 621,
                                    end: 622,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        18446744073709551615,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 625,
                                    end: 706,
                                    as_str(): "0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                24,
                            ),
                            type_ascription: TypeId(
                                24,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 617,
                    end: 707,
                    as_str(): "let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 716,
                                    end: 717,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                            255,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    14,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359303f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                                    ),
                                    start: 720,
                                    end: 1041,
                                    as_str(): "0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                14,
                            ),
                            type_ascription: TypeId(
                                25,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 712,
                    end: 1042,
                    as_str(): "let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;",
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
                            3,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1359303f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                            ),
                            start: 1048,
                            end: 1052,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb1359303f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                    ),
                    start: 1048,
                    end: 1052,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb1359303f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
        ),
        start: 9,
        end: 1054,
        as_str(): "fn main() -> bool {\n    // Hex\n    let x = 0xF;\n    let x = 0xFF;\n    let x = 0xFFF;\n    let x = 0xAAAA;\n    let x = 0xB_3333;\n    let x = 0xFFFF_4444;\n    let x = 0x1111_1111_1111;\n    let x = 0xFFFF_FFFFFFFFFFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n\n    // Binary\n    let y = 0b1;\n    let y = 0b11;\n    let y = 0b111;\n    let y = 0b1111;\n    let y = 0b1_1111;\n    let y = 0b1111_0000;\n    let y = 0b1111_1111_1111;\n    let y = 0b1111_111111111111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n\n    true\n}",
    },
    attributes: {},
    return_type: TypeId(
        3,
    ),
    initial_return_type: TypeId(
        3,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb1359303f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
        ),
        start: 22,
        end: 26,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

