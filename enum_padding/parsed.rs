[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0ee12f040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                            ),
                            start: 18,
                            end: 32,
                            as_str(): "LowerLevelEnum",
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
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 39,
                                    end: 44,
                                    as_str(): "first",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: B256,
                            type_span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 46,
                                end: 50,
                                as_str(): "b256",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 39,
                                end: 50,
                                as_str(): "first: b256",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 56,
                                    end: 62,
                                    as_str(): "second",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                ThirtyTwo,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 64,
                                end: 67,
                                as_str(): "u32",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 56,
                                end: 67,
                                as_str(): "second: u32",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb0ee12f040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                        ),
                        start: 9,
                        end: 70,
                        as_str(): "pub enum LowerLevelEnum {\n    first: b256,\n    second: u32,\n}",
                    },
                    visibility: Public,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ee12f040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
            ),
            start: 9,
            end: 70,
            as_str(): "pub enum LowerLevelEnum {\n    first: b256,\n    second: u32,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0ee12f040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                            ),
                            start: 83,
                            end: 94,
                            as_str(): "ThenAStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 101,
                                    end: 106,
                                    as_str(): "first",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                ThirtyTwo,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 101,
                                end: 111,
                                as_str(): "first: u32",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 108,
                                end: 111,
                                as_str(): "u32",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 117,
                                    end: 123,
                                    as_str(): "second",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 125,
                                        end: 139,
                                        as_str(): "LowerLevelEnum",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 117,
                                end: 139,
                                as_str(): "second: LowerLevelEnum",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 125,
                                end: 139,
                                as_str(): "LowerLevelEnum",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Public,
                    span: Span {
                        src (ptr): 0x00007fb0ee12f040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                        ),
                        start: 72,
                        end: 142,
                        as_str(): "pub struct ThenAStruct {\n    first: u32,\n    second: LowerLevelEnum,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ee12f040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
            ),
            start: 72,
            end: 142,
            as_str(): "pub struct ThenAStruct {\n    first: u32,\n    second: LowerLevelEnum,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0ee12f040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                            ),
                            start: 153,
                            end: 165,
                            as_str(): "TopLevelEnum",
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
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 172,
                                    end: 177,
                                    as_str(): "first",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [
                                    TypeArgument {
                                        type_id: TypeId(
                                            0,
                                        ),
                                        initial_type_id: TypeId(
                                            0,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 180,
                                            end: 184,
                                            as_str(): "b256",
                                        },
                                    },
                                    TypeArgument {
                                        type_id: TypeId(
                                            0,
                                        ),
                                        initial_type_id: TypeId(
                                            0,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 190,
                                            end: 194,
                                            as_str(): "b256",
                                        },
                                    },
                                ],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 179,
                                end: 195,
                                as_str(): "(b256,\n    b256)",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 172,
                                end: 195,
                                as_str(): "first: (b256,\n    b256)",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 197,
                                    end: 203,
                                    as_str(): "second",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 205,
                                        end: 216,
                                        as_str(): "ThenAStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 205,
                                end: 216,
                                as_str(): "ThenAStruct",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 197,
                                end: 216,
                                as_str(): "second: ThenAStruct",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb0ee12f040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                        ),
                        start: 144,
                        end: 219,
                        as_str(): "pub enum TopLevelEnum {\n    first: (b256,\n    b256), second: ThenAStruct,\n}",
                    },
                    visibility: Public,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ee12f040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
            ),
            start: 144,
            end: 219,
            as_str(): "pub enum TopLevelEnum {\n    first: (b256,\n    b256), second: ThenAStruct,\n}",
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
                            src (ptr): 0x00007fb0ee12f040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                            ),
                            start: 224,
                            end: 228,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
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
                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                        ),
                                                                        start: 840,
                                                                        end: 852,
                                                                        as_str(): "TopLevelEnum",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                    ),
                                                                    start: 840,
                                                                    end: 852,
                                                                    as_str(): "TopLevelEnum",
                                                                },
                                                            },
                                                            suffix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                    ),
                                                                    start: 854,
                                                                    end: 860,
                                                                    as_str(): "second",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 840,
                                                        end: 860,
                                                        as_str(): "TopLevelEnum::second",
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
                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                ),
                                                                                start: 861,
                                                                                end: 872,
                                                                                as_str(): "ThenAStruct",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                        ),
                                                                        start: 861,
                                                                        end: 872,
                                                                        as_str(): "ThenAStruct",
                                                                    },
                                                                },
                                                                fields: [
                                                                    StructExpressionField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                ),
                                                                                start: 883,
                                                                                end: 888,
                                                                                as_str(): "first",
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
                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                ),
                                                                                start: 890,
                                                                                end: 892,
                                                                                as_str(): "42",
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 883,
                                                                            end: 892,
                                                                            as_str(): "first: 42",
                                                                        },
                                                                    },
                                                                    StructExpressionField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                ),
                                                                                start: 894,
                                                                                end: 900,
                                                                                as_str(): "second",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value: Expression {
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
                                                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                            ),
                                                                                                            start: 902,
                                                                                                            end: 916,
                                                                                                            as_str(): "LowerLevelEnum",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                        ),
                                                                                                        start: 902,
                                                                                                        end: 916,
                                                                                                        as_str(): "LowerLevelEnum",
                                                                                                    },
                                                                                                },
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                        ),
                                                                                                        start: 918,
                                                                                                        end: 924,
                                                                                                        as_str(): "second",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                            ),
                                                                                            start: 902,
                                                                                            end: 924,
                                                                                            as_str(): "LowerLevelEnum::second",
                                                                                        },
                                                                                    },
                                                                                    args: [
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    66,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                ),
                                                                                                start: 925,
                                                                                                end: 927,
                                                                                                as_str(): "66",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                ),
                                                                                start: 902,
                                                                                end: 928,
                                                                                as_str(): "LowerLevelEnum::second(66)",
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 894,
                                                                            end: 928,
                                                                            as_str(): "second: LowerLevelEnum::second(66)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ee12f040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                            ),
                                                            start: 861,
                                                            end: 934,
                                                            as_str(): "ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 840,
                                            end: 935,
                                            as_str(): "TopLevelEnum::second(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 840,
                                    end: 935,
                                    as_str(): "TopLevelEnum::second(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0ee12f040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                            ),
                            start: 247,
                            end: 937,
                            as_str(): "{\n    // Expected output:\n    //\n    //  0000000000000001  # TopLevelEnum.tag\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  000000000000002a  #     ThenAStruct.first(42)\n    //  0000000000000001  #     ThenAStruct.LowerLevelEnum.tag\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000042  #         ThenAStruct.LowerLevelEnum.second(66)\n\n    TopLevelEnum::second(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0ee12f040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                        ),
                        start: 221,
                        end: 937,
                        as_str(): "fn main() -> TopLevelEnum {\n    // Expected output:\n    //\n    //  0000000000000001  # TopLevelEnum.tag\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  000000000000002a  #     ThenAStruct.first(42)\n    //  0000000000000001  #     ThenAStruct.LowerLevelEnum.tag\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000042  #         ThenAStruct.LowerLevelEnum.second(66)\n\n    TopLevelEnum::second(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 234,
                                end: 246,
                                as_str(): "TopLevelEnum",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0ee12f040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                        ),
                        start: 234,
                        end: 246,
                        as_str(): "TopLevelEnum",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ee12f040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
            ),
            start: 221,
            end: 937,
            as_str(): "fn main() -> TopLevelEnum {\n    // Expected output:\n    //\n    //  0000000000000001  # TopLevelEnum.tag\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  000000000000002a  #     ThenAStruct.first(42)\n    //  0000000000000001  #     ThenAStruct.LowerLevelEnum.tag\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000042  #         ThenAStruct.LowerLevelEnum.second(66)\n\n    TopLevelEnum::second(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })\n}",
        },
    },
]
