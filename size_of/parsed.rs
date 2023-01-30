[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 18,
                            end: 24,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 26,
                            end: 32,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe04328bd30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
            ),
            start: 9,
            end: 33,
            as_str(): "use std::assert::assert;",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 78,
                            end: 82,
                            as_str(): "Data",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 87,
                                    end: 90,
                                    as_str(): "one",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 87,
                                end: 95,
                                as_str(): "one: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 92,
                                end: 95,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 99,
                                    end: 102,
                                    as_str(): "two",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 99,
                                end: 107,
                                as_str(): "two: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 104,
                                end: 107,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 111,
                                    end: 116,
                                    as_str(): "three",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 111,
                                end: 121,
                                as_str(): "three: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 118,
                                end: 121,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe04328bd30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                        ),
                        start: 71,
                        end: 124,
                        as_str(): "struct Data {\n  one: u64,\n  two: u64,\n  three: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04328bd30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
            ),
            start: 71,
            end: 124,
            as_str(): "struct Data {\n  one: u64,\n  two: u64,\n  three: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 170,
                            end: 175,
                            as_str(): "Point",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 180,
                                    end: 181,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                Eight,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 180,
                                end: 185,
                                as_str(): "x: u8",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 183,
                                end: 185,
                                as_str(): "u8",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 189,
                                    end: 190,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                Eight,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 189,
                                end: 194,
                                as_str(): "y: u8",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 192,
                                end: 194,
                                as_str(): "u8",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 198,
                                    end: 199,
                                    as_str(): "z",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                Eight,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 198,
                                end: 203,
                                as_str(): "z: u8",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 201,
                                end: 203,
                                as_str(): "u8",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe04328bd30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                        ),
                        start: 163,
                        end: 206,
                        as_str(): "struct Point {\n  x: u8,\n  y: u8,\n  z: u8,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04328bd30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
            ),
            start: 163,
            end: 206,
            as_str(): "struct Point {\n  x: u8,\n  y: u8,\n  z: u8,\n}",
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
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 211,
                            end: 226,
                            as_str(): "return_the_same",
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
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 252,
                                                    end: 253,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 255,
                                                        end: 256,
                                                        as_str(): "T",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 255,
                                                    end: 256,
                                                    as_str(): "T",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 263,
                                                            as_str(): "elem",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 259,
                                                    end: 263,
                                                    as_str(): "elem",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 248,
                                    end: 264,
                                    as_str(): "let x: T = elem;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 267,
                                                    end: 268,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 267,
                                            end: 268,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 267,
                                    end: 268,
                                    as_str(): "x",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 244,
                            end: 270,
                            as_str(): "{\n  let x: T = elem;\n  x\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 230,
                                    end: 234,
                                    as_str(): "elem",
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
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 236,
                                        end: 237,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 236,
                                end: 237,
                                as_str(): "T",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe04328bd30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                        ),
                        start: 208,
                        end: 270,
                        as_str(): "fn return_the_same<T>(elem: T) -> T {\n  let x: T = elem;\n  x\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe04328bd30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                ),
                                start: 242,
                                end: 243,
                                as_str(): "T",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        T: TypeId(31628),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe04328bd30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                        ),
                        start: 242,
                        end: 243,
                        as_str(): "T",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04328bd30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
            ),
            start: 208,
            end: 270,
            as_str(): "fn return_the_same<T>(elem: T) -> T {\n  let x: T = elem;\n  x\n}",
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
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 275,
                            end: 279,
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
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 299,
                                                    end: 300,
                                                    as_str(): "x",
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 303,
                                                                        end: 307,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 303,
                                                                end: 307,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 318,
                                                                        end: 321,
                                                                        as_str(): "one",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 323,
                                                                        end: 324,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 318,
                                                                    end: 324,
                                                                    as_str(): "one: 1",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 334,
                                                                        end: 337,
                                                                        as_str(): "two",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 339,
                                                                        end: 340,
                                                                        as_str(): "2",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 334,
                                                                    end: 340,
                                                                    as_str(): "two: 2",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 350,
                                                                        end: 355,
                                                                        as_str(): "three",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 357,
                                                                        end: 358,
                                                                        as_str(): "3",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 350,
                                                                    end: 358,
                                                                    as_str(): "three: 3",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 303,
                                                    end: 365,
                                                    as_str(): "Data {\n        one: 1,\n        two: 2,\n        three: 3,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 295,
                                    end: 366,
                                    as_str(): "let x = Data {\n        one: 1,\n        two: 2,\n        three: 3,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 375,
                                                    end: 376,
                                                    as_str(): "y",
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 379,
                                                                        end: 383,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 379,
                                                                end: 383,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 394,
                                                                        end: 397,
                                                                        as_str(): "one",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            10000,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 399,
                                                                        end: 404,
                                                                        as_str(): "10000",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 394,
                                                                    end: 404,
                                                                    as_str(): "one: 10000",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 414,
                                                                        end: 417,
                                                                        as_str(): "two",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            20000,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 419,
                                                                        end: 424,
                                                                        as_str(): "20000",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 414,
                                                                    end: 424,
                                                                    as_str(): "two: 20000",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 434,
                                                                        end: 439,
                                                                        as_str(): "three",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            30000,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 441,
                                                                        end: 446,
                                                                        as_str(): "30000",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 434,
                                                                    end: 446,
                                                                    as_str(): "three: 30000",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 379,
                                                    end: 453,
                                                    as_str(): "Data {\n        one: 10000,\n        two: 20000,\n        three: 30000,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 371,
                                    end: 454,
                                    as_str(): "let y = Data {\n        one: 10000,\n        two: 20000,\n        three: 30000,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 463,
                                                    end: 464,
                                                    as_str(): "p",
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 467,
                                                                        end: 472,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 467,
                                                                end: 472,
                                                                as_str(): "Point",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 481,
                                                                        end: 482,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 484,
                                                                        end: 485,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 481,
                                                                    end: 485,
                                                                    as_str(): "x: 0",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 493,
                                                                        end: 494,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 496,
                                                                        end: 497,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 493,
                                                                    end: 497,
                                                                    as_str(): "y: 1",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 505,
                                                                        end: 506,
                                                                        as_str(): "z",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 508,
                                                                        end: 509,
                                                                        as_str(): "2",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 505,
                                                                    end: 509,
                                                                    as_str(): "z: 2",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 467,
                                                    end: 515,
                                                    as_str(): "Point {\n      x: 0,\n      y: 1,\n      z: 2\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 459,
                                    end: 516,
                                    as_str(): "let p = Point {\n      x: 0,\n      y: 1,\n      z: 2\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 525,
                                                    end: 528,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 531,
                                                                        end: 546,
                                                                        as_str(): "return_the_same",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 531,
                                                                end: 546,
                                                                as_str(): "return_the_same",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        7,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 547,
                                                                    end: 551,
                                                                    as_str(): "7u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 531,
                                                    end: 552,
                                                    as_str(): "return_the_same(7u64)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 521,
                                    end: 553,
                                    as_str(): "let foo = return_the_same(7u64);",
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 558,
                                                                end: 564,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 558,
                                                        end: 564,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 582,
                                                                                        end: 584,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 582,
                                                                                        end: 584,
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
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 582,
                                                                                    end: 584,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 582,
                                                                        end: 584,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 565,
                                                                                        end: 578,
                                                                                        as_str(): "__size_of_val",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: SizeOfVal,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 565,
                                                                                        end: 581,
                                                                                        as_str(): "__size_of_val(x)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 579,
                                                                                                    end: 580,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 579,
                                                                                            end: 580,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 565,
                                                                            end: 581,
                                                                            as_str(): "__size_of_val(x)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                24,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 585,
                                                                            end: 587,
                                                                            as_str(): "24",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 565,
                                                            end: 587,
                                                            as_str(): "__size_of_val(x) == 24",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 558,
                                            end: 588,
                                            as_str(): "assert(__size_of_val(x) == 24)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 558,
                                    end: 588,
                                    as_str(): "assert(__size_of_val(x) == 24)",
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 594,
                                                                end: 600,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 594,
                                                        end: 600,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 618,
                                                                                        end: 620,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 618,
                                                                                        end: 620,
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
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 618,
                                                                                    end: 620,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 618,
                                                                        end: 620,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 601,
                                                                                        end: 614,
                                                                                        as_str(): "__size_of_val",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: SizeOfVal,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 601,
                                                                                        end: 617,
                                                                                        as_str(): "__size_of_val(y)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 615,
                                                                                                    end: 616,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 615,
                                                                                            end: 616,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 601,
                                                                            end: 617,
                                                                            as_str(): "__size_of_val(y)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                24,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 621,
                                                                            end: 623,
                                                                            as_str(): "24",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 601,
                                                            end: 623,
                                                            as_str(): "__size_of_val(y) == 24",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 594,
                                            end: 624,
                                            as_str(): "assert(__size_of_val(y) == 24)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 594,
                                    end: 624,
                                    as_str(): "assert(__size_of_val(y) == 24)",
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 630,
                                                                end: 636,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 630,
                                                        end: 636,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 657,
                                                                                        end: 659,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 657,
                                                                                        end: 659,
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
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 657,
                                                                                    end: 659,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 657,
                                                                        end: 659,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 637,
                                                                                        end: 646,
                                                                                        as_str(): "__size_of",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: SizeOfType,
                                                                                    type_arguments: [
                                                                                        TypeArgument {
                                                                                            type_id: TypeId(
                                                                                                31629,
                                                                                            ),
                                                                                            initial_type_id: TypeId(
                                                                                                31629,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                ),
                                                                                                start: 649,
                                                                                                end: 653,
                                                                                                as_str(): "Data",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 637,
                                                                                        end: 656,
                                                                                        as_str(): "__size_of::<Data>()",
                                                                                    },
                                                                                },
                                                                                arguments: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 637,
                                                                            end: 656,
                                                                            as_str(): "__size_of::<Data>()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                24,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 660,
                                                                            end: 662,
                                                                            as_str(): "24",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 637,
                                                            end: 662,
                                                            as_str(): "__size_of::<Data>() == 24",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 630,
                                            end: 663,
                                            as_str(): "assert(__size_of::<Data>() == 24)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 630,
                                    end: 663,
                                    as_str(): "assert(__size_of::<Data>() == 24)",
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 669,
                                                                end: 675,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 669,
                                                        end: 675,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 693,
                                                                                        end: 695,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 693,
                                                                                        end: 695,
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
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 693,
                                                                                    end: 695,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 693,
                                                                        end: 695,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 676,
                                                                                        end: 689,
                                                                                        as_str(): "__size_of_val",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: SizeOfVal,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 676,
                                                                                        end: 692,
                                                                                        as_str(): "__size_of_val(p)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 690,
                                                                                                    end: 691,
                                                                                                    as_str(): "p",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 690,
                                                                                            end: 691,
                                                                                            as_str(): "p",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 676,
                                                                            end: 692,
                                                                            as_str(): "__size_of_val(p)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                24,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 696,
                                                                            end: 698,
                                                                            as_str(): "24",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 676,
                                                            end: 698,
                                                            as_str(): "__size_of_val(p) == 24",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 669,
                                            end: 699,
                                            as_str(): "assert(__size_of_val(p) == 24)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 669,
                                    end: 699,
                                    as_str(): "assert(__size_of_val(p) == 24)",
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 705,
                                                                end: 711,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 705,
                                                        end: 711,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 731,
                                                                                        end: 733,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 731,
                                                                                        end: 733,
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
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 731,
                                                                                    end: 733,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 731,
                                                                        end: 733,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 712,
                                                                                        end: 725,
                                                                                        as_str(): "__size_of_val",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: SizeOfVal,
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 712,
                                                                                        end: 730,
                                                                                        as_str(): "__size_of_val(foo)",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 726,
                                                                                                    end: 729,
                                                                                                    as_str(): "foo",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 726,
                                                                                            end: 729,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 712,
                                                                            end: 730,
                                                                            as_str(): "__size_of_val(foo)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                8,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 734,
                                                                            end: 735,
                                                                            as_str(): "8",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 712,
                                                            end: 735,
                                                            as_str(): "__size_of_val(foo) == 8",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 705,
                                            end: 736,
                                            as_str(): "assert(__size_of_val(foo) == 8)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 705,
                                    end: 736,
                                    as_str(): "assert(__size_of_val(foo) == 8)",
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 742,
                                                                end: 748,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 742,
                                                        end: 748,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 770,
                                                                                        end: 772,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 770,
                                                                                        end: 772,
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
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 770,
                                                                                    end: 772,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 770,
                                                                        end: 772,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 749,
                                                                                        end: 758,
                                                                                        as_str(): "__size_of",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: SizeOfType,
                                                                                    type_arguments: [
                                                                                        TypeArgument {
                                                                                            type_id: TypeId(
                                                                                                31630,
                                                                                            ),
                                                                                            initial_type_id: TypeId(
                                                                                                31630,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                ),
                                                                                                start: 761,
                                                                                                end: 766,
                                                                                                as_str(): "Point",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 749,
                                                                                        end: 769,
                                                                                        as_str(): "__size_of::<Point>()",
                                                                                    },
                                                                                },
                                                                                arguments: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 749,
                                                                            end: 769,
                                                                            as_str(): "__size_of::<Point>()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                24,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 773,
                                                                            end: 775,
                                                                            as_str(): "24",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 749,
                                                            end: 775,
                                                            as_str(): "__size_of::<Point>() == 24",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 742,
                                            end: 776,
                                            as_str(): "assert(__size_of::<Point>() == 24)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 742,
                                    end: 776,
                                    as_str(): "assert(__size_of::<Point>() == 24)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 782,
                                            end: 783,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04328bd30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                    ),
                                    start: 782,
                                    end: 783,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 289,
                            end: 785,
                            as_str(): "{\n    let x = Data {\n        one: 1,\n        two: 2,\n        three: 3,\n    };\n    let y = Data {\n        one: 10000,\n        two: 20000,\n        three: 30000,\n    };\n    let p = Point {\n      x: 0,\n      y: 1,\n      z: 2\n    };\n    let foo = return_the_same(7u64);\n    assert(__size_of_val(x) == 24);\n    assert(__size_of_val(y) == 24);\n    assert(__size_of::<Data>() == 24);\n    assert(__size_of_val(p) == 24);\n    assert(__size_of_val(foo) == 8);\n    assert(__size_of::<Point>() == 24);\n    1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe04328bd30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                        ),
                        start: 272,
                        end: 785,
                        as_str(): "fn main() -> u64 {\n    let x = Data {\n        one: 1,\n        two: 2,\n        three: 3,\n    };\n    let y = Data {\n        one: 10000,\n        two: 20000,\n        three: 30000,\n    };\n    let p = Point {\n      x: 0,\n      y: 1,\n      z: 2\n    };\n    let foo = return_the_same(7u64);\n    assert(__size_of_val(x) == 24);\n    assert(__size_of_val(y) == 24);\n    assert(__size_of::<Data>() == 24);\n    assert(__size_of_val(p) == 24);\n    assert(__size_of_val(foo) == 8);\n    assert(__size_of::<Point>() == 24);\n    1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe04328bd30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                        ),
                        start: 285,
                        end: 288,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04328bd30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
            ),
            start: 272,
            end: 785,
            as_str(): "fn main() -> u64 {\n    let x = Data {\n        one: 1,\n        two: 2,\n        three: 3,\n    };\n    let y = Data {\n        one: 10000,\n        two: 20000,\n        three: 30000,\n    };\n    let p = Point {\n      x: 0,\n      y: 1,\n      z: 2\n    };\n    let foo = return_the_same(7u64);\n    assert(__size_of_val(x) == 24);\n    assert(__size_of_val(y) == 24);\n    assert(__size_of::<Data>() == 24);\n    assert(__size_of_val(p) == 24);\n    assert(__size_of_val(foo) == 8);\n    assert(__size_of::<Point>() == 24);\n    1\n}",
        },
    },
]
