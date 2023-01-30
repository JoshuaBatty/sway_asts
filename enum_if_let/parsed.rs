[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
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
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 19,
                            end: 26,
                            as_str(): "address",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 28,
                            end: 35,
                            as_str(): "Address",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 9,
            end: 73,
            as_str(): "use std::{address::Address, assert::assert, identity::Identity};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
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
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 37,
                            end: 43,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 45,
                            end: 51,
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
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 9,
            end: 73,
            as_str(): "use std::{address::Address, assert::assert, identity::Identity};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
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
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 53,
                            end: 61,
                            as_str(): "identity",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 63,
                            end: 71,
                            as_str(): "Identity",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 9,
            end: 73,
            as_str(): "use std::{address::Address, assert::assert, identity::Identity};",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 80,
                            end: 86,
                            as_str(): "Result",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(31628),
                        E: TypeId(31629),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 99,
                                    end: 101,
                                    as_str(): "Ok",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 104,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0f64c4390,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                ),
                                start: 103,
                                end: 104,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb0f64c4390,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                ),
                                start: 99,
                                end: 104,
                                as_str(): "Ok: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 110,
                                    end: 113,
                                    as_str(): "Err",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 115,
                                        end: 116,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0f64c4390,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                ),
                                start: 115,
                                end: 116,
                                as_str(): "E",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb0f64c4390,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                ),
                                start: 110,
                                end: 116,
                                as_str(): "Err: E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb0f64c4390,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                        ),
                        start: 75,
                        end: 119,
                        as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 75,
            end: 119,
            as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 127,
                            end: 129,
                            as_str(): "B1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Struct(
                            StructExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 132,
                                                end: 139,
                                                as_str(): "Address",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 132,
                                        end: 139,
                                        as_str(): "Address",
                                    },
                                },
                                fields: [
                                    StructExpressionField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 146,
                                                end: 151,
                                                as_str(): "value",
                                            },
                                            is_raw_ident: false,
                                        },
                                        value: Expression {
                                            kind: Literal(
                                                B256(
                                                    [
                                                        1,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        16,
                                                    ],
                                                ),
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 153,
                                                end: 219,
                                                as_str(): "0x0100000000000000000000000000000000000000000000000000000000000010",
                                            },
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 146,
                                            end: 219,
                                            as_str(): "value: 0x0100000000000000000000000000000000000000000000000000000000000010",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 132,
                            end: 221,
                            as_str(): "Address {\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n}",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb0f64c4390,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                        ),
                        start: 121,
                        end: 222,
                        as_str(): "const B1 = Address {\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n};",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 121,
            end: 222,
            as_str(): "const B1 = Address {\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n};",
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
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 227,
                            end: 231,
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
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 251,
                                                    end: 252,
                                                    as_str(): "a",
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
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 255,
                                                                                end: 261,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 255,
                                                                            end: 261,
                                                                            as_str(): "Result",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 263,
                                                                            end: 265,
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
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 268,
                                                                        end: 271,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 273,
                                                                        end: 276,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 255,
                                                                end: 277,
                                                                as_str(): "Result::Ok::<u64, u64>",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        100,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 278,
                                                                    end: 281,
                                                                    as_str(): "100",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 255,
                                                    end: 282,
                                                    as_str(): "Result::Ok::<u64, u64>(100)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 247,
                                    end: 283,
                                    as_str(): "let a = Result::Ok::<u64, u64>(100);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 292,
                                                    end: 293,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Match(
                                                    MatchExpression {
                                                        value: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 319,
                                                                        end: 320,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 319,
                                                                end: 320,
                                                                as_str(): "a",
                                                            },
                                                        },
                                                        branches: [
                                                            MatchBranch {
                                                                scrutinee: EnumScrutinee {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 303,
                                                                                    end: 309,
                                                                                    as_str(): "Result",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 311,
                                                                                end: 313,
                                                                                as_str(): "Ok",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    value: Variable {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 314,
                                                                                end: 315,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 314,
                                                                            end: 315,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 303,
                                                                        end: 316,
                                                                        as_str(): "Result::Ok(y)",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
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
                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 325,
                                                                                                                            end: 326,
                                                                                                                            as_str(): "+",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 325,
                                                                                                                            end: 326,
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
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 325,
                                                                                                                        end: 326,
                                                                                                                        as_str(): "+",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                is_absolute: true,
                                                                                                            },
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 325,
                                                                                                            end: 326,
                                                                                                            as_str(): "+",
                                                                                                        },
                                                                                                    },
                                                                                                    contract_call_params: [],
                                                                                                    arguments: [
                                                                                                        Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 323,
                                                                                                                        end: 324,
                                                                                                                        as_str(): "y",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 323,
                                                                                                                end: 324,
                                                                                                                as_str(): "y",
                                                                                                            },
                                                                                                        },
                                                                                                        Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    10,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 327,
                                                                                                                end: 329,
                                                                                                                as_str(): "10",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 323,
                                                                                                end: 329,
                                                                                                as_str(): "y + 10",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 323,
                                                                                        end: 329,
                                                                                        as_str(): "y + 10",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 321,
                                                                                end: 331,
                                                                                as_str(): "{ y + 10 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 321,
                                                                        end: 331,
                                                                        as_str(): "{ y + 10 }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 303,
                                                                    end: 331,
                                                                    as_str(): "Result::Ok(y) = a { y + 10 }",
                                                                },
                                                            },
                                                            MatchBranch {
                                                                scrutinee: CatchAll {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 337,
                                                                        end: 342,
                                                                        as_str(): "{ 1 }",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    1,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 339,
                                                                                                end: 340,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 339,
                                                                                        end: 340,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 337,
                                                                                end: 342,
                                                                                as_str(): "{ 1 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 337,
                                                                        end: 342,
                                                                        as_str(): "{ 1 }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 337,
                                                                    end: 342,
                                                                    as_str(): "{ 1 }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 296,
                                                    end: 342,
                                                    as_str(): "if let Result::Ok(y) = a { y + 10 } else { 1 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 288,
                                    end: 343,
                                    as_str(): "let b = if let Result::Ok(y) = a { y + 10 } else { 1 };",
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
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 348,
                                                                end: 354,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 348,
                                                        end: 354,
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
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 357,
                                                                                        end: 359,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 357,
                                                                                        end: 359,
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
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 357,
                                                                                    end: 359,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 357,
                                                                        end: 359,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 355,
                                                                                    end: 356,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 355,
                                                                            end: 356,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                110,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 360,
                                                                            end: 363,
                                                                            as_str(): "110",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 355,
                                                            end: 363,
                                                            as_str(): "b == 110",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 348,
                                            end: 364,
                                            as_str(): "assert(b == 110)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 348,
                                    end: 364,
                                    as_str(): "assert(b == 110)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 375,
                                                    end: 381,
                                                    as_str(): "sender",
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
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 384,
                                                                                end: 392,
                                                                                as_str(): "Identity",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 384,
                                                                            end: 392,
                                                                            as_str(): "Identity",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 394,
                                                                            end: 401,
                                                                            as_str(): "Address",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 384,
                                                                end: 401,
                                                                as_str(): "Identity::Address",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 402,
                                                                            end: 404,
                                                                            as_str(): "B1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 402,
                                                                    end: 404,
                                                                    as_str(): "B1",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 384,
                                                    end: 405,
                                                    as_str(): "Identity::Address(B1)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 371,
                                    end: 406,
                                    as_str(): "let sender = Identity::Address(B1);",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Match(
                                            MatchExpression {
                                                value: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 445,
                                                                end: 451,
                                                                as_str(): "sender",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 445,
                                                        end: 451,
                                                        as_str(): "sender",
                                                    },
                                                },
                                                branches: [
                                                    MatchBranch {
                                                        scrutinee: EnumScrutinee {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 418,
                                                                            end: 426,
                                                                            as_str(): "Identity",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 428,
                                                                        end: 435,
                                                                        as_str(): "Address",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            value: Variable {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 436,
                                                                        end: 441,
                                                                        as_str(): "addr1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 436,
                                                                    end: 441,
                                                                    as_str(): "addr1",
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 418,
                                                                end: 442,
                                                                as_str(): "Identity::Address(addr1)",
                                                            },
                                                        },
                                                        result: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [
                                                                                                AstNode {
                                                                                                    content: Declaration(
                                                                                                        VariableDeclaration(
                                                                                                            VariableDeclaration {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 468,
                                                                                                                        end: 474,
                                                                                                                        as_str(): "sender",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                type_ascription: Unknown,
                                                                                                                type_ascription_span: None,
                                                                                                                body: Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 468,
                                                                                                                                end: 474,
                                                                                                                                as_str(): "sender",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 468,
                                                                                                                        end: 474,
                                                                                                                        as_str(): "sender",
                                                                                                                    },
                                                                                                                },
                                                                                                                is_mutable: false,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 462,
                                                                                                        end: 646,
                                                                                                        as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                                                                    },
                                                                                                },
                                                                                                AstNode {
                                                                                                    content: ImplicitReturnExpression(
                                                                                                        Expression {
                                                                                                            kind: Match(
                                                                                                                MatchExpression {
                                                                                                                    value: Expression {
                                                                                                                        kind: Variable(
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "__match_return_var_name_1",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 468,
                                                                                                                                    end: 474,
                                                                                                                                    as_str(): "sender",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 468,
                                                                                                                            end: 474,
                                                                                                                            as_str(): "sender",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    branches: [
                                                                                                                        MatchBranch {
                                                                                                                            scrutinee: EnumScrutinee {
                                                                                                                                call_path: CallPath {
                                                                                                                                    prefixes: [
                                                                                                                                        BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 489,
                                                                                                                                                end: 497,
                                                                                                                                                as_str(): "Identity",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    ],
                                                                                                                                    suffix: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 499,
                                                                                                                                            end: 506,
                                                                                                                                            as_str(): "Address",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    is_absolute: false,
                                                                                                                                },
                                                                                                                                value: Variable {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 507,
                                                                                                                                            end: 512,
                                                                                                                                            as_str(): "addr2",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 507,
                                                                                                                                        end: 512,
                                                                                                                                        as_str(): "addr2",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 489,
                                                                                                                                    end: 513,
                                                                                                                                    as_str(): "Identity::Address(addr2)",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            result: Expression {
                                                                                                                                kind: CodeBlock(
                                                                                                                                    CodeBlock {
                                                                                                                                        contents: [
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
                                                                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 535,
                                                                                                                                                                                end: 541,
                                                                                                                                                                                as_str(): "assert",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                        is_absolute: false,
                                                                                                                                                                    },
                                                                                                                                                                    type_arguments: [],
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 535,
                                                                                                                                                                        end: 541,
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
                                                                                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                                        path: Some(
                                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                                        ),
                                                                                                                                                                                                        start: 548,
                                                                                                                                                                                                        end: 550,
                                                                                                                                                                                                        as_str(): "==",
                                                                                                                                                                                                    },
                                                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                                                },
                                                                                                                                                                                                BaseIdent {
                                                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                                                        "ops",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    span: Span {
                                                                                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                                        path: Some(
                                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                                        ),
                                                                                                                                                                                                        start: 548,
                                                                                                                                                                                                        end: 550,
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
                                                                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 548,
                                                                                                                                                                                                    end: 550,
                                                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                                                },
                                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                                            },
                                                                                                                                                                                            is_absolute: true,
                                                                                                                                                                                        },
                                                                                                                                                                                    },
                                                                                                                                                                                    type_arguments: [],
                                                                                                                                                                                    span: Span {
                                                                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                        path: Some(
                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                        ),
                                                                                                                                                                                        start: 548,
                                                                                                                                                                                        end: 550,
                                                                                                                                                                                        as_str(): "==",
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                                contract_call_params: [],
                                                                                                                                                                                arguments: [
                                                                                                                                                                                    Expression {
                                                                                                                                                                                        kind: Variable(
                                                                                                                                                                                            BaseIdent {
                                                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                                                span: Span {
                                                                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 542,
                                                                                                                                                                                                    end: 547,
                                                                                                                                                                                                    as_str(): "addr1",
                                                                                                                                                                                                },
                                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                                            },
                                                                                                                                                                                        ),
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 542,
                                                                                                                                                                                            end: 547,
                                                                                                                                                                                            as_str(): "addr1",
                                                                                                                                                                                        },
                                                                                                                                                                                    },
                                                                                                                                                                                    Expression {
                                                                                                                                                                                        kind: Variable(
                                                                                                                                                                                            BaseIdent {
                                                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                                                span: Span {
                                                                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 551,
                                                                                                                                                                                                    end: 556,
                                                                                                                                                                                                    as_str(): "addr2",
                                                                                                                                                                                                },
                                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                                            },
                                                                                                                                                                                        ),
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 551,
                                                                                                                                                                                            end: 556,
                                                                                                                                                                                            as_str(): "addr2",
                                                                                                                                                                                        },
                                                                                                                                                                                    },
                                                                                                                                                                                ],
                                                                                                                                                                            },
                                                                                                                                                                        ),
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 542,
                                                                                                                                                                            end: 556,
                                                                                                                                                                            as_str(): "addr1 == addr2",
                                                                                                                                                                        },
                                                                                                                                                                    },
                                                                                                                                                                ],
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 535,
                                                                                                                                                            end: 557,
                                                                                                                                                            as_str(): "assert(addr1 == addr2)",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 535,
                                                                                                                                                    end: 557,
                                                                                                                                                    as_str(): "assert(addr1 == addr2)",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ],
                                                                                                                                        whole_block_span: Span {
                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 517,
                                                                                                                                            end: 572,
                                                                                                                                            as_str(): "{\n                assert(addr1 == addr2);\n            }",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 517,
                                                                                                                                    end: 572,
                                                                                                                                    as_str(): "{\n                assert(addr1 == addr2);\n            }",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 489,
                                                                                                                                end: 572,
                                                                                                                                as_str(): "Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        MatchBranch {
                                                                                                                            scrutinee: CatchAll {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 585,
                                                                                                                                    end: 586,
                                                                                                                                    as_str(): "_",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            result: Expression {
                                                                                                                                kind: CodeBlock(
                                                                                                                                    CodeBlock {
                                                                                                                                        contents: [
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
                                                                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 608,
                                                                                                                                                                                end: 614,
                                                                                                                                                                                as_str(): "assert",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                        is_absolute: false,
                                                                                                                                                                    },
                                                                                                                                                                    type_arguments: [],
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 608,
                                                                                                                                                                        end: 614,
                                                                                                                                                                        as_str(): "assert",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                arguments: [
                                                                                                                                                                    Expression {
                                                                                                                                                                        kind: Literal(
                                                                                                                                                                            Boolean(
                                                                                                                                                                                false,
                                                                                                                                                                            ),
                                                                                                                                                                        ),
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 615,
                                                                                                                                                                            end: 620,
                                                                                                                                                                            as_str(): "false",
                                                                                                                                                                        },
                                                                                                                                                                    },
                                                                                                                                                                ],
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 608,
                                                                                                                                                            end: 621,
                                                                                                                                                            as_str(): "assert(false)",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 608,
                                                                                                                                                    end: 621,
                                                                                                                                                    as_str(): "assert(false)",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ],
                                                                                                                                        whole_block_span: Span {
                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 590,
                                                                                                                                            end: 636,
                                                                                                                                            as_str(): "{\n                assert(false);\n            }",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 590,
                                                                                                                                    end: 636,
                                                                                                                                    as_str(): "{\n                assert(false);\n            }",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 585,
                                                                                                                                end: 636,
                                                                                                                                as_str(): "_ => {\n                assert(false);\n            }",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 462,
                                                                                                                end: 646,
                                                                                                                as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 462,
                                                                                                        end: 646,
                                                                                                        as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 462,
                                                                                                end: 646,
                                                                                                as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 462,
                                                                                        end: 646,
                                                                                        as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 462,
                                                                                end: 646,
                                                                                as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 452,
                                                                        end: 652,
                                                                        as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 452,
                                                                end: 652,
                                                                as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 418,
                                                            end: 652,
                                                            as_str(): "Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                                        },
                                                    },
                                                    MatchBranch {
                                                        scrutinee: CatchAll {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 452,
                                                                end: 652,
                                                                as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                                            },
                                                        },
                                                        result: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 452,
                                                                        end: 652,
                                                                        as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 452,
                                                                end: 652,
                                                                as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 452,
                                                            end: 652,
                                                            as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 411,
                                            end: 652,
                                            as_str(): "if let Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 411,
                                    end: 652,
                                    as_str(): "if let Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 663,
                                                    end: 664,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 666,
                                                        end: 672,
                                                        as_str(): "Result",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                21,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 673,
                                                                end: 676,
                                                                as_str(): "u64",
                                                            },
                                                        },
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                21,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 678,
                                                                end: 681,
                                                                as_str(): "u64",
                                                            },
                                                        },
                                                    ],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 666,
                                                    end: 682,
                                                    as_str(): "Result<u64, u64>",
                                                },
                                            ),
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
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 685,
                                                                                end: 691,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 685,
                                                                            end: 691,
                                                                            as_str(): "Result",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
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
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 698,
                                                                        end: 701,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 703,
                                                                        end: 706,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 685,
                                                                end: 707,
                                                                as_str(): "Result::Ok::<u64, u64>",
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
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 708,
                                                                    end: 712,
                                                                    as_str(): "5u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 685,
                                                    end: 713,
                                                    as_str(): "Result::Ok::<u64, u64>(5u64)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 659,
                                    end: 714,
                                    as_str(): "let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 724,
                                                    end: 732,
                                                    as_str(): "result_1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Match(
                                                    MatchExpression {
                                                        value: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 759,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 758,
                                                                end: 759,
                                                                as_str(): "x",
                                                            },
                                                        },
                                                        branches: [
                                                            MatchBranch {
                                                                scrutinee: EnumScrutinee {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 742,
                                                                                    end: 748,
                                                                                    as_str(): "Result",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 750,
                                                                                end: 752,
                                                                                as_str(): "Ok",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    value: Variable {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 753,
                                                                                end: 754,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 753,
                                                                            end: 754,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 742,
                                                                        end: 755,
                                                                        as_str(): "Result::Ok(x)",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    100,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 770,
                                                                                                end: 773,
                                                                                                as_str(): "100",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 770,
                                                                                        end: 773,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 760,
                                                                                end: 779,
                                                                                as_str(): "{\n        100\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 760,
                                                                        end: 779,
                                                                        as_str(): "{\n        100\n    }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 742,
                                                                    end: 779,
                                                                    as_str(): "Result::Ok(x) = x {\n        100\n    }",
                                                                },
                                                            },
                                                            MatchBranch {
                                                                scrutinee: CatchAll {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 785,
                                                                        end: 802,
                                                                        as_str(): "{\n        1\n    }",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    1,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 795,
                                                                                                end: 796,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 795,
                                                                                        end: 796,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 785,
                                                                                end: 802,
                                                                                as_str(): "{\n        1\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 785,
                                                                        end: 802,
                                                                        as_str(): "{\n        1\n    }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 785,
                                                                    end: 802,
                                                                    as_str(): "{\n        1\n    }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 735,
                                                    end: 802,
                                                    as_str(): "if let Result::Ok(x) = x {\n        100\n    } else {\n        1\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 720,
                                    end: 803,
                                    as_str(): "let result_1 = if let Result::Ok(x) = x {\n        100\n    } else {\n        1\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 812,
                                                    end: 820,
                                                    as_str(): "result_2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Match(
                                                    MatchExpression {
                                                        value: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 847,
                                                                        end: 848,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 847,
                                                                end: 848,
                                                                as_str(): "x",
                                                            },
                                                        },
                                                        branches: [
                                                            MatchBranch {
                                                                scrutinee: EnumScrutinee {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 830,
                                                                                    end: 836,
                                                                                    as_str(): "Result",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 838,
                                                                                end: 841,
                                                                                as_str(): "Err",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    value: Variable {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 842,
                                                                                end: 843,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 842,
                                                                            end: 843,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 830,
                                                                        end: 844,
                                                                        as_str(): "Result::Err(x)",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    3,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 859,
                                                                                                end: 860,
                                                                                                as_str(): "3",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 859,
                                                                                        end: 860,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 849,
                                                                                end: 866,
                                                                                as_str(): "{\n        3\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 849,
                                                                        end: 866,
                                                                        as_str(): "{\n        3\n    }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 830,
                                                                    end: 866,
                                                                    as_str(): "Result::Err(x) = x {\n        3\n    }",
                                                                },
                                                            },
                                                            MatchBranch {
                                                                scrutinee: CatchAll {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 872,
                                                                        end: 890,
                                                                        as_str(): "{\n        43\n    }",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    43,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 882,
                                                                                                end: 884,
                                                                                                as_str(): "43",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 882,
                                                                                        end: 884,
                                                                                        as_str(): "43",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 872,
                                                                                end: 890,
                                                                                as_str(): "{\n        43\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 872,
                                                                        end: 890,
                                                                        as_str(): "{\n        43\n    }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 872,
                                                                    end: 890,
                                                                    as_str(): "{\n        43\n    }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 823,
                                                    end: 890,
                                                    as_str(): "if let Result::Err(x) = x {\n        3\n    } else {\n        43\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 808,
                                    end: 891,
                                    as_str(): "let result_2 = if let Result::Err(x) = x {\n        3\n    } else {\n        43\n    };",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
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
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 905,
                                                                        end: 906,
                                                                        as_str(): "+",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "ops",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 905,
                                                                        end: 906,
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
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 905,
                                                                    end: 906,
                                                                    as_str(): "+",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_absolute: true,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 905,
                                                        end: 906,
                                                        as_str(): "+",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 896,
                                                                    end: 904,
                                                                    as_str(): "result_1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 896,
                                                            end: 904,
                                                            as_str(): "result_1",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 907,
                                                                    end: 915,
                                                                    as_str(): "result_2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 907,
                                                            end: 915,
                                                            as_str(): "result_2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 896,
                                            end: 915,
                                            as_str(): "result_1 + result_2",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 896,
                                    end: 915,
                                    as_str(): "result_1 + result_2",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 241,
                            end: 917,
                            as_str(): "{\n    let a = Result::Ok::<u64, u64>(100);\n    let b = if let Result::Ok(y) = a { y + 10 } else { 1 };\n    assert(b == 110);\n\n    let sender = Identity::Address(B1);\n    if let Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    };\n\n    let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    let result_1 = if let Result::Ok(x) = x {\n        100\n    } else {\n        1\n    };\n    let result_2 = if let Result::Err(x) = x {\n        3\n    } else {\n        43\n    };\n    result_1 + result_2\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0f64c4390,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                        ),
                        start: 224,
                        end: 917,
                        as_str(): "fn main() -> u64 {\n    let a = Result::Ok::<u64, u64>(100);\n    let b = if let Result::Ok(y) = a { y + 10 } else { 1 };\n    assert(b == 110);\n\n    let sender = Identity::Address(B1);\n    if let Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    };\n\n    let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    let result_1 = if let Result::Ok(x) = x {\n        100\n    } else {\n        1\n    };\n    let result_2 = if let Result::Err(x) = x {\n        3\n    } else {\n        43\n    };\n    result_1 + result_2\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0f64c4390,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                        ),
                        start: 237,
                        end: 240,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 224,
            end: 917,
            as_str(): "fn main() -> u64 {\n    let a = Result::Ok::<u64, u64>(100);\n    let b = if let Result::Ok(y) = a { y + 10 } else { 1 };\n    assert(b == 110);\n\n    let sender = Identity::Address(B1);\n    if let Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    };\n\n    let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    let result_1 = if let Result::Ok(x) = x {\n        100\n    } else {\n        1\n    };\n    let result_2 = if let Result::Err(x) = x {\n        3\n    } else {\n        43\n    };\n    result_1 + result_2\n}",
        },
    },
]
