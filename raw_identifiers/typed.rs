TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06737e0f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
            ),
            start: 14,
            end: 22,
            as_str(): "SomeEnum",
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
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 29,
                    end: 30,
                    as_str(): "B",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fe06737e0f0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                ),
                start: 32,
                end: 36,
                as_str(): "bool",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe06737e0f0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                ),
                start: 29,
                end: 36,
                as_str(): "B: bool",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe06737e0f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
        ),
        start: 9,
        end: 39,
        as_str(): "enum SomeEnum {\n    B: bool,\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06737e0f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
            ),
            start: 44,
            end: 48,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 74,
                                    end: 80,
                                    as_str(): "script",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 83,
                                    end: 84,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7252,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 64,
                    end: 85,
                    as_str(): "let mut r#script = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 100,
                                    end: 108,
                                    as_str(): "contract",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 111,
                                    end: 112,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7254,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 90,
                    end: 113,
                    as_str(): "let mut r#contract = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 128,
                                    end: 137,
                                    as_str(): "predicate",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 140,
                                    end: 141,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7256,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 118,
                    end: 142,
                    as_str(): "let mut r#predicate = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 157,
                                    end: 164,
                                    as_str(): "library",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 167,
                                    end: 168,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7258,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 147,
                    end: 169,
                    as_str(): "let mut r#library = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 184,
                                    end: 187,
                                    as_str(): "dep",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 190,
                                    end: 191,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7260,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 174,
                    end: 192,
                    as_str(): "let mut r#dep = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 207,
                                    end: 210,
                                    as_str(): "pub",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 213,
                                    end: 214,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7262,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 197,
                    end: 215,
                    as_str(): "let mut r#pub = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 230,
                                    end: 233,
                                    as_str(): "use",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 236,
                                    end: 237,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7264,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 220,
                    end: 238,
                    as_str(): "let mut r#use = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 253,
                                    end: 255,
                                    as_str(): "as",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 258,
                                    end: 259,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7266,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 243,
                    end: 260,
                    as_str(): "let mut r#as = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 275,
                                    end: 281,
                                    as_str(): "struct",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 284,
                                    end: 285,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7268,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 265,
                    end: 286,
                    as_str(): "let mut r#struct = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 301,
                                    end: 305,
                                    as_str(): "enum",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 308,
                                    end: 309,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7270,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 291,
                    end: 310,
                    as_str(): "let mut r#enum = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 325,
                                    end: 329,
                                    as_str(): "self",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 332,
                                    end: 333,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7272,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 315,
                    end: 334,
                    as_str(): "let mut r#self = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 349,
                                    end: 351,
                                    as_str(): "fn",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 354,
                                    end: 355,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7274,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 339,
                    end: 356,
                    as_str(): "let mut r#fn = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 371,
                                    end: 376,
                                    as_str(): "trait",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 379,
                                    end: 380,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7276,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 361,
                    end: 381,
                    as_str(): "let mut r#trait = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 396,
                                    end: 400,
                                    as_str(): "impl",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 403,
                                    end: 404,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7278,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 386,
                    end: 405,
                    as_str(): "let mut r#impl = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 420,
                                    end: 423,
                                    as_str(): "for",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 426,
                                    end: 427,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7280,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 410,
                    end: 428,
                    as_str(): "let mut r#for = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 443,
                                    end: 446,
                                    as_str(): "abi",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 449,
                                    end: 450,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7282,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 433,
                    end: 451,
                    as_str(): "let mut r#abi = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 466,
                                    end: 471,
                                    as_str(): "const",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 474,
                                    end: 475,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7284,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 456,
                    end: 476,
                    as_str(): "let mut r#const = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 491,
                                    end: 498,
                                    as_str(): "storage",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 501,
                                    end: 502,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7286,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 481,
                    end: 503,
                    as_str(): "let mut r#storage = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 518,
                                    end: 521,
                                    as_str(): "str",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 524,
                                    end: 525,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7288,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 508,
                    end: 526,
                    as_str(): "let mut r#str = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 541,
                                    end: 544,
                                    as_str(): "asm",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 547,
                                    end: 548,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7290,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 531,
                    end: 549,
                    as_str(): "let mut r#asm = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 564,
                                    end: 570,
                                    as_str(): "return",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 573,
                                    end: 574,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7292,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 554,
                    end: 575,
                    as_str(): "let mut r#return = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 590,
                                    end: 592,
                                    as_str(): "if",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 595,
                                    end: 596,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7294,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 580,
                    end: 597,
                    as_str(): "let mut r#if = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 612,
                                    end: 616,
                                    as_str(): "else",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 619,
                                    end: 620,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7296,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 602,
                    end: 621,
                    as_str(): "let mut r#else = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 636,
                                    end: 641,
                                    as_str(): "match",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 644,
                                    end: 645,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7298,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 626,
                    end: 646,
                    as_str(): "let mut r#match = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 661,
                                    end: 664,
                                    as_str(): "mut",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 667,
                                    end: 668,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7300,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 651,
                    end: 669,
                    as_str(): "let mut r#mut = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 684,
                                    end: 687,
                                    as_str(): "let",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 690,
                                    end: 691,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7302,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 674,
                    end: 692,
                    as_str(): "let mut r#let = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 707,
                                    end: 712,
                                    as_str(): "while",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 715,
                                    end: 716,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7304,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 697,
                    end: 717,
                    as_str(): "let mut r#while = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 732,
                                    end: 737,
                                    as_str(): "where",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 740,
                                    end: 741,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7306,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 722,
                    end: 742,
                    as_str(): "let mut r#where = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 757,
                                    end: 760,
                                    as_str(): "ref",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 763,
                                    end: 764,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7308,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 747,
                    end: 765,
                    as_str(): "let mut r#ref = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 780,
                                    end: 785,
                                    as_str(): "deref",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 788,
                                    end: 789,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7310,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 770,
                    end: 790,
                    as_str(): "let mut r#deref = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 805,
                                    end: 809,
                                    as_str(): "true",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 812,
                                    end: 813,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7312,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 795,
                    end: 814,
                    as_str(): "let mut r#true = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 829,
                                    end: 834,
                                    as_str(): "false",
                                },
                                is_raw_ident: true,
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
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 837,
                                    end: 838,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7314,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 819,
                    end: 839,
                    as_str(): "let mut r#false = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 849,
                                    end: 850,
                                    as_str(): "e",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe06737e0f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                ),
                                                start: 14,
                                                end: 22,
                                                as_str(): "SomeEnum",
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
                                                        src (ptr): 0x00007fe06737e0f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                        ),
                                                        start: 29,
                                                        end: 30,
                                                        as_str(): "B",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    71,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe06737e0f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                    ),
                                                    start: 32,
                                                    end: 36,
                                                    as_str(): "bool",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe06737e0f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                    ),
                                                    start: 29,
                                                    end: 36,
                                                    as_str(): "B: bool",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe06737e0f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 39,
                                            as_str(): "enum SomeEnum {\n    B: bool,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06737e0f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                            ),
                                            start: 29,
                                            end: 30,
                                            as_str(): "B",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
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
                                                src (ptr): 0x00007fe06737e0f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                ),
                                                start: 865,
                                                end: 870,
                                                as_str(): "false",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe06737e0f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                        ),
                                        start: 853,
                                        end: 861,
                                        as_str(): "SomeEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe06737e0f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                        ),
                                        start: 863,
                                        end: 864,
                                        as_str(): "B",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe06737e0f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                            ),
                                            start: 853,
                                            end: 864,
                                            as_str(): "SomeEnum::B",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7317,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 863,
                                    end: 864,
                                    as_str(): "B",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7317,
                            ),
                            type_ascription: TypeId(
                                7316,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 845,
                    end: 872,
                    as_str(): "let e = SomeEnum::B(false);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 881,
                                    end: 882,
                                    as_str(): "v",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Declaration(
                                                    VariableDeclaration(
                                                        TyVariableDeclaration {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "__match_return_var_name_1",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 891,
                                                                    end: 892,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                            ),
                                                                            start: 849,
                                                                            end: 850,
                                                                            as_str(): "e",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                        ),
                                                                        start: 891,
                                                                        end: 892,
                                                                        as_str(): "e",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7317,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 891,
                                                                    end: 892,
                                                                    as_str(): "e",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7317,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7320,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06737e0f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                    ),
                                                    start: 885,
                                                    end: 1013,
                                                    as_str(): "match e {\n        SomeEnum::B(true) => {\n            1\n        },\n        SomeEnum::B(false) => {\n            0\n        },\n    }",
                                                },
                                            },
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: IfExp {
                                                            condition: TyExpression {
                                                                expression: LazyOperator {
                                                                    op: And,
                                                                    lhs: TyExpression {
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 891,
                                                                                            end: 892,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 891,
                                                                                            end: 892,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "eq",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                        ),
                                                                                        start: 891,
                                                                                        end: 892,
                                                                                        as_str(): "e",
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
                                                                                            src (ptr): 0x00007fe065a784d0,
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
                                                                                        expression: EnumTag {
                                                                                            exp: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_1",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                            ),
                                                                                                            start: 891,
                                                                                                            end: 892,
                                                                                                            as_str(): "e",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 891,
                                                                                                        end: 892,
                                                                                                        as_str(): "e",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7317,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 891,
                                                                                                    end: 892,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 891,
                                                                                            end: 892,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe065a784d0,
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
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 891,
                                                                                            end: 892,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                549,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe065a784d0,
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
                                                                            type_binding: None,
                                                                        },
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                            ),
                                                                            start: 891,
                                                                            end: 892,
                                                                            as_str(): "e",
                                                                        },
                                                                    },
                                                                    rhs: TyExpression {
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 903,
                                                                                            end: 920,
                                                                                            as_str(): "SomeEnum::B(true)",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 903,
                                                                                            end: 920,
                                                                                            as_str(): "SomeEnum::B(true)",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "eq",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                        ),
                                                                                        start: 903,
                                                                                        end: 920,
                                                                                        as_str(): "SomeEnum::B(true)",
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
                                                                                            src (ptr): 0x00007fe065a784d0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 2930,
                                                                                            end: 2934,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: UnsafeDowncast {
                                                                                            exp: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_1",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                            ),
                                                                                                            start: 891,
                                                                                                            end: 892,
                                                                                                            as_str(): "e",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 891,
                                                                                                        end: 892,
                                                                                                        as_str(): "e",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7317,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 891,
                                                                                                    end: 892,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                            },
                                                                                            variant: TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 29,
                                                                                                        end: 30,
                                                                                                        as_str(): "B",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    71,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    71,
                                                                                                ),
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 32,
                                                                                                    end: 36,
                                                                                                    as_str(): "bool",
                                                                                                },
                                                                                                tag: 0,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 29,
                                                                                                    end: 36,
                                                                                                    as_str(): "B: bool",
                                                                                                },
                                                                                                attributes: {},
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            71,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 903,
                                                                                            end: 920,
                                                                                            as_str(): "SomeEnum::B(true)",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe065a784d0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 2936,
                                                                                            end: 2941,
                                                                                            as_str(): "other",
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
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 915,
                                                                                            end: 919,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                548,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe065a784d0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 2924,
                                                                                    end: 2990,
                                                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: None,
                                                                        },
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                            ),
                                                                            start: 903,
                                                                            end: 920,
                                                                            as_str(): "SomeEnum::B(true)",
                                                                        },
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 891,
                                                                    end: 920,
                                                                    as_str(): "e {\n        SomeEnum::B(true)",
                                                                },
                                                            },
                                                            then: TyExpression {
                                                                expression: CodeBlock(
                                                                    TyCodeBlock {
                                                                        contents: [
                                                                            TyAstNode {
                                                                                content: ImplicitReturnExpression(
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
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 938,
                                                                                            end: 939,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                    ),
                                                                                    start: 938,
                                                                                    end: 939,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 924,
                                                                    end: 949,
                                                                    as_str(): "{\n            1\n        }",
                                                                },
                                                            },
                                                            else: Some(
                                                                TyExpression {
                                                                    expression: IfExp {
                                                                        condition: TyExpression {
                                                                            expression: LazyOperator {
                                                                                op: And,
                                                                                lhs: TyExpression {
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "core",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 891,
                                                                                                        end: 892,
                                                                                                        as_str(): "e",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 891,
                                                                                                        end: 892,
                                                                                                        as_str(): "e",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "eq",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 891,
                                                                                                    end: 892,
                                                                                                    as_str(): "e",
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
                                                                                                        src (ptr): 0x00007fe065a784d0,
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
                                                                                                    expression: EnumTag {
                                                                                                        exp: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 891,
                                                                                                                        end: 892,
                                                                                                                        as_str(): "e",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 891,
                                                                                                                    end: 892,
                                                                                                                    as_str(): "e",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7317,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                ),
                                                                                                                start: 891,
                                                                                                                end: 892,
                                                                                                                as_str(): "e",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 891,
                                                                                                        end: 892,
                                                                                                        as_str(): "e",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe065a784d0,
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
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 891,
                                                                                                        end: 892,
                                                                                                        as_str(): "e",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            547,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe065a784d0,
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
                                                                                        type_binding: None,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                        ),
                                                                                        start: 891,
                                                                                        end: 892,
                                                                                        as_str(): "e",
                                                                                    },
                                                                                },
                                                                                rhs: TyExpression {
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "core",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 959,
                                                                                                        end: 977,
                                                                                                        as_str(): "SomeEnum::B(false)",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 959,
                                                                                                        end: 977,
                                                                                                        as_str(): "SomeEnum::B(false)",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "eq",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 959,
                                                                                                    end: 977,
                                                                                                    as_str(): "SomeEnum::B(false)",
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
                                                                                                        src (ptr): 0x00007fe065a784d0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 2930,
                                                                                                        end: 2934,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: UnsafeDowncast {
                                                                                                        exp: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 891,
                                                                                                                        end: 892,
                                                                                                                        as_str(): "e",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 891,
                                                                                                                    end: 892,
                                                                                                                    as_str(): "e",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7317,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                ),
                                                                                                                start: 891,
                                                                                                                end: 892,
                                                                                                                as_str(): "e",
                                                                                                            },
                                                                                                        },
                                                                                                        variant: TyEnumVariant {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 29,
                                                                                                                    end: 30,
                                                                                                                    as_str(): "B",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_id: TypeId(
                                                                                                                71,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                71,
                                                                                                            ),
                                                                                                            type_span: Span {
                                                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                ),
                                                                                                                start: 32,
                                                                                                                end: 36,
                                                                                                                as_str(): "bool",
                                                                                                            },
                                                                                                            tag: 0,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                                ),
                                                                                                                start: 29,
                                                                                                                end: 36,
                                                                                                                as_str(): "B: bool",
                                                                                                            },
                                                                                                            attributes: {},
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        71,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 959,
                                                                                                        end: 977,
                                                                                                        as_str(): "SomeEnum::B(false)",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe065a784d0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 2936,
                                                                                                        end: 2941,
                                                                                                        as_str(): "other",
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
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 971,
                                                                                                        end: 976,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            546,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe065a784d0,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                ),
                                                                                                start: 2924,
                                                                                                end: 2990,
                                                                                                as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: None,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                        ),
                                                                                        start: 959,
                                                                                        end: 977,
                                                                                        as_str(): "SomeEnum::B(false)",
                                                                                    },
                                                                                },
                                                                            },
                                                                            return_type: TypeId(
                                                                                71,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                ),
                                                                                start: 891,
                                                                                end: 977,
                                                                                as_str(): "e {\n        SomeEnum::B(true) => {\n            1\n        },\n        SomeEnum::B(false)",
                                                                            },
                                                                        },
                                                                        then: TyExpression {
                                                                            expression: CodeBlock(
                                                                                TyCodeBlock {
                                                                                    contents: [
                                                                                        TyAstNode {
                                                                                            content: ImplicitReturnExpression(
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
                                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                        ),
                                                                                                        start: 995,
                                                                                                        end: 996,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                ),
                                                                                                start: 995,
                                                                                                end: 996,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                ),
                                                                                start: 981,
                                                                                end: 1006,
                                                                                as_str(): "{\n            0\n        }",
                                                                            },
                                                                        },
                                                                        else: Some(
                                                                            TyExpression {
                                                                                expression: CodeBlock(
                                                                                    TyCodeBlock {
                                                                                        contents: [
                                                                                            TyAstNode {
                                                                                                content: ImplicitReturnExpression(
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
                                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                            ),
                                                                                                            start: 995,
                                                                                                            end: 996,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 995,
                                                                                                    end: 996,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                    ),
                                                                                    start: 981,
                                                                                    end: 1006,
                                                                                    as_str(): "{\n            0\n        }",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                        ),
                                                                        start: 981,
                                                                        end: 1006,
                                                                        as_str(): "{\n            0\n        }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 924,
                                                            end: 949,
                                                            as_str(): "{\n            1\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06737e0f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                    ),
                                                    start: 885,
                                                    end: 1013,
                                                    as_str(): "match e {\n        SomeEnum::B(true) => {\n            1\n        },\n        SomeEnum::B(false) => {\n            0\n        },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06737e0f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                    ),
                                    start: 885,
                                    end: 1013,
                                    as_str(): "match e {\n        SomeEnum::B(true) => {\n            1\n        },\n        SomeEnum::B(false) => {\n            0\n        },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7319,
                            ),
                            type_ascription: TypeId(
                                7319,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 877,
                    end: 1014,
                    as_str(): "let v = match e {\n        SomeEnum::B(true) => {\n            1\n        },\n        SomeEnum::B(false) => {\n            0\n        },\n    };",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                0,
                            ),
                        ),
                        return_type: TypeId(
                            7326,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06737e0f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                            ),
                            start: 1020,
                            end: 1021,
                            as_str(): "0",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06737e0f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                    ),
                    start: 1020,
                    end: 1021,
                    as_str(): "0",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06737e0f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
        ),
        start: 41,
        end: 1023,
        as_str(): "fn main() -> u64 {\n    let mut r#script = 0;\n    let mut r#contract = 0;\n    let mut r#predicate = 0;\n    let mut r#library = 0;\n    let mut r#dep = 0;\n    let mut r#pub = 0;\n    let mut r#use = 0;\n    let mut r#as = 0;\n    let mut r#struct = 0;\n    let mut r#enum = 0;\n    let mut r#self = 0;\n    let mut r#fn = 0;\n    let mut r#trait = 0;\n    let mut r#impl = 0;\n    let mut r#for = 0;\n    let mut r#abi = 0;\n    let mut r#const = 0;\n    let mut r#storage = 0;\n    let mut r#str = 0;\n    let mut r#asm = 0;\n    let mut r#return = 0;\n    let mut r#if = 0;\n    let mut r#else = 0;\n    let mut r#match = 0;\n    let mut r#mut = 0;\n    let mut r#let = 0;\n    let mut r#while = 0;\n    let mut r#where = 0;\n    let mut r#ref = 0;\n    let mut r#deref = 0;\n    let mut r#true = 0;\n    let mut r#false = 0;\n\n    let e = SomeEnum::B(false);\n    let v = match e {\n        SomeEnum::B(true) => {\n            1\n        },\n        SomeEnum::B(false) => {\n            0\n        },\n    };\n\n    0\n}",
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
        src (ptr): 0x00007fe06737e0f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
        ),
        start: 54,
        end: 57,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

