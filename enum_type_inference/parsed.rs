[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 175,
                            end: 181,
                            as_str(): "Result",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(7249),
                        E: TypeId(7250),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 192,
                                    end: 194,
                                    as_str(): "Ok",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 196,
                                        end: 197,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0ed8e1980,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                ),
                                start: 196,
                                end: 197,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb0ed8e1980,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                ),
                                start: 192,
                                end: 197,
                                as_str(): "Ok: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 201,
                                    end: 204,
                                    as_str(): "Err",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 206,
                                        end: 207,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0ed8e1980,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                ),
                                start: 206,
                                end: 207,
                                as_str(): "E",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb0ed8e1980,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                ),
                                start: 201,
                                end: 207,
                                as_str(): "Err: E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb0ed8e1980,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                        ),
                        start: 170,
                        end: 210,
                        as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 170,
            end: 210,
            as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 219,
                            end: 226,
                            as_str(): "Product",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb0ed8e1980,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                        ),
                        start: 212,
                        end: 230,
                        as_str(): "struct Product {\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 212,
            end: 230,
            as_str(): "struct Product {\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 239,
                            end: 250,
                            as_str(): "ItemDetails",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb0ed8e1980,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                        ),
                        start: 232,
                        end: 254,
                        as_str(): "struct ItemDetails {\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 232,
            end: 254,
            as_str(): "struct ItemDetails {\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 261,
                            end: 270,
                            as_str(): "SaleError",
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
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 277,
                                    end: 295,
                                    as_str(): "NotEnoughInventory",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Str(
                                Length {
                                    val: 3,
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 301,
                                        end: 302,
                                        as_str(): "3",
                                    },
                                },
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb0ed8e1980,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                ),
                                start: 297,
                                end: 303,
                                as_str(): "str[3]",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb0ed8e1980,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                ),
                                start: 277,
                                end: 303,
                                as_str(): "NotEnoughInventory: str[3]",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb0ed8e1980,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                        ),
                        start: 256,
                        end: 307,
                        as_str(): "enum SaleError {\n    NotEnoughInventory: str[3], \n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 256,
            end: 307,
            as_str(): "enum SaleError {\n    NotEnoughInventory: str[3], \n}",
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
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 312,
                            end: 316,
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
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 336,
                                                    end: 337,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 340,
                                                                                end: 346,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 340,
                                                                            end: 346,
                                                                            as_str(): "Result",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 348,
                                                                            end: 350,
                                                                            as_str(): "Ok",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ed8e1980,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 353,
                                                                        end: 356,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        7251,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        7251,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ed8e1980,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 358,
                                                                        end: 367,
                                                                        as_str(): "SaleError",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 340,
                                                                end: 368,
                                                                as_str(): "Result::Ok::<u64, SaleError>",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        5,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 369,
                                                                    end: 373,
                                                                    as_str(): "5u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 340,
                                                    end: 374,
                                                    as_str(): "Result::Ok::<u64, SaleError>(5u64)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 332,
                                    end: 375,
                                    as_str(): "let x = Result::Ok::<u64, SaleError>(5u64);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 388,
                                                    end: 389,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 392,
                                                                                end: 398,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 392,
                                                                            end: 398,
                                                                            as_str(): "Result",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 400,
                                                                            end: 403,
                                                                            as_str(): "Err",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ed8e1980,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 406,
                                                                        end: 409,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        7252,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        7252,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ed8e1980,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 411,
                                                                        end: 420,
                                                                        as_str(): "SaleError",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 392,
                                                                end: 421,
                                                                as_str(): "Result::Err::<u64, SaleError>",
                                                            },
                                                        },
                                                        args: [
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
                                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 422,
                                                                                                end: 431,
                                                                                                as_str(): "SaleError",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 422,
                                                                                            end: 431,
                                                                                            as_str(): "SaleError",
                                                                                        },
                                                                                    },
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 433,
                                                                                            end: 451,
                                                                                            as_str(): "NotEnoughInventory",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 422,
                                                                                end: 451,
                                                                                as_str(): "SaleError::NotEnoughInventory",
                                                                            },
                                                                        },
                                                                        args: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    String(
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 453,
                                                                                            end: 456,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 452,
                                                                                    end: 457,
                                                                                    as_str(): "\"foo\"",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 422,
                                                                    end: 458,
                                                                    as_str(): "SaleError::NotEnoughInventory(\"foo\")",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 392,
                                                    end: 459,
                                                    as_str(): "Result::Err::<u64, SaleError>(SaleError::NotEnoughInventory(\"foo\"))",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 380,
                                    end: 460,
                                    as_str(): "let mut y = Result::Err::<u64, SaleError>(SaleError::NotEnoughInventory(\"foo\"));",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Reassignment(
                                            ReassignmentExpression {
                                                lhs: VariableExpression(
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 496,
                                                                    end: 497,
                                                                    as_str(): "y",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 496,
                                                            end: 497,
                                                            as_str(): "y",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 500,
                                                                end: 501,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 500,
                                                        end: 501,
                                                        as_str(): "x",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 496,
                                            end: 501,
                                            as_str(): "y = x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 496,
                                    end: 501,
                                    as_str(): "y = x",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                5,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 507,
                                            end: 508,
                                            as_str(): "5",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 507,
                                    end: 508,
                                    as_str(): "5",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 326,
                            end: 510,
                            as_str(): "{\n    let x = Result::Ok::<u64, SaleError>(5u64);\n    let mut y = Result::Err::<u64, SaleError>(SaleError::NotEnoughInventory(\"foo\"));\n    // should be the same type\n    y = x;\n    5\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0ed8e1980,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                        ),
                        start: 309,
                        end: 510,
                        as_str(): "fn main() -> u64 {\n    let x = Result::Ok::<u64, SaleError>(5u64);\n    let mut y = Result::Err::<u64, SaleError>(SaleError::NotEnoughInventory(\"foo\"));\n    // should be the same type\n    y = x;\n    5\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0ed8e1980,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                        ),
                        start: 322,
                        end: 325,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 309,
            end: 510,
            as_str(): "fn main() -> u64 {\n    let x = Result::Ok::<u64, SaleError>(5u64);\n    let mut y = Result::Err::<u64, SaleError>(SaleError::NotEnoughInventory(\"foo\"));\n    // should be the same type\n    y = x;\n    5\n}",
        },
    },
]
