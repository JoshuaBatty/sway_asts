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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        15,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        255,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        4095,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        43690,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        734003,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        4294919236,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        18764998447377,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        18446744073709551615,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        1,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        3,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        7,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        15,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        31,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        240,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        4095,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        65535,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        18446744073709551615,
                                                    ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
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
                                            is_mutable: false,
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1359303f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                            ),
                            start: 27,
                            end: 1054,
                            as_str(): "{\n    // Hex\n    let x = 0xF;\n    let x = 0xFF;\n    let x = 0xFFF;\n    let x = 0xAAAA;\n    let x = 0xB_3333;\n    let x = 0xFFFF_4444;\n    let x = 0x1111_1111_1111;\n    let x = 0xFFFF_FFFFFFFFFFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n\n    // Binary\n    let y = 0b1;\n    let y = 0b11;\n    let y = 0b111;\n    let y = 0b1111;\n    let y = 0b1_1111;\n    let y = 0b1111_0000;\n    let y = 0b1111_1111_1111;\n    let y = 0b1111_111111111111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb1359303f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
                        ),
                        start: 9,
                        end: 1054,
                        as_str(): "fn main() -> bool {\n    // Hex\n    let x = 0xF;\n    let x = 0xFF;\n    let x = 0xFFF;\n    let x = 0xAAAA;\n    let x = 0xB_3333;\n    let x = 0xFFFF_4444;\n    let x = 0x1111_1111_1111;\n    let x = 0xFFFF_FFFFFFFFFFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n\n    // Binary\n    let y = 0b1;\n    let y = 0b11;\n    let y = 0b111;\n    let y = 0b1111;\n    let y = 0b1_1111;\n    let y = 0b1111_0000;\n    let y = 0b1111_1111_1111;\n    let y = 0b1111_111111111111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n\n    true\n}",
                    },
                    return_type: Boolean,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1359303f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNQVsN0/binary_and_hex_literals/src/main.sw",
            ),
            start: 9,
            end: 1054,
            as_str(): "fn main() -> bool {\n    // Hex\n    let x = 0xF;\n    let x = 0xFF;\n    let x = 0xFFF;\n    let x = 0xAAAA;\n    let x = 0xB_3333;\n    let x = 0xFFFF_4444;\n    let x = 0x1111_1111_1111;\n    let x = 0xFFFF_FFFFFFFFFFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n    let x = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;\n\n    // Binary\n    let y = 0b1;\n    let y = 0b11;\n    let y = 0b111;\n    let y = 0b1111;\n    let y = 0b1_1111;\n    let y = 0b1111_0000;\n    let y = 0b1111_1111_1111;\n    let y = 0b1111_111111111111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n    let y = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;\n\n    true\n}",
        },
    },
]
