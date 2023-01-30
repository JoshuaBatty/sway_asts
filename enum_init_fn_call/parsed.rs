[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
            src (ptr): 0x00007fb0f2710960,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 42,
                            end: 44,
                            as_str(): "T1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 51,
                                    end: 53,
                                    as_str(): "t1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 51,
                                end: 58,
                                as_str(): "t1: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 55,
                                end: 58,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 35,
                        end: 62,
                        as_str(): "struct T1 {\n    t1: u64, \n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f2710960,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
            ),
            start: 35,
            end: 62,
            as_str(): "struct T1 {\n    t1: u64, \n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 71,
                            end: 73,
                            as_str(): "T2",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 80,
                                    end: 82,
                                    as_str(): "t1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 80,
                                end: 87,
                                as_str(): "t1: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 84,
                                end: 87,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 96,
                                    as_str(): "t2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 94,
                                end: 101,
                                as_str(): "t2: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 98,
                                end: 101,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 64,
                        end: 103,
                        as_str(): "struct T2 {\n    t1: u64, \n    t2: u64\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f2710960,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
            ),
            start: 64,
            end: 103,
            as_str(): "struct T2 {\n    t1: u64, \n    t2: u64\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 110,
                            end: 111,
                            as_str(): "A",
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
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 118,
                                    end: 119,
                                    as_str(): "A",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 121,
                                end: 124,
                                as_str(): "u64",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 118,
                                end: 124,
                                as_str(): "A: u64",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 130,
                                    end: 131,
                                    as_str(): "B",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 133,
                                        end: 135,
                                        as_str(): "T1",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 133,
                                end: 135,
                                as_str(): "T1",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 130,
                                end: 135,
                                as_str(): "B: T1",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 141,
                                    end: 142,
                                    as_str(): "C",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 144,
                                        end: 146,
                                        as_str(): "T2",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 144,
                                end: 146,
                                as_str(): "T2",
                            },
                            tag: 2,
                            span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 141,
                                end: 146,
                                as_str(): "C: T2",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 105,
                        end: 149,
                        as_str(): "enum A {\n    A: u64,\n    B: T1,\n    C: T2,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f2710960,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
            ),
            start: 105,
            end: 149,
            as_str(): "enum A {\n    A: u64,\n    B: T1,\n    C: T2,\n}",
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
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 154,
                            end: 158,
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
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 178,
                                                    end: 179,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Match(
                                                    MatchExpression {
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
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 199,
                                                                                            end: 200,
                                                                                            as_str(): "A",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 199,
                                                                                        end: 200,
                                                                                        as_str(): "A",
                                                                                    },
                                                                                },
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 202,
                                                                                        end: 203,
                                                                                        as_str(): "A",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 199,
                                                                            end: 203,
                                                                            as_str(): "A::A",
                                                                        },
                                                                    },
                                                                    args: [
                                                                        Expression {
                                                                            kind: FunctionApplication(
                                                                                FunctionApplicationExpression {
                                                                                    call_path_binding: TypeBinding {
                                                                                        inner: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                    ),
                                                                                                    start: 204,
                                                                                                    end: 205,
                                                                                                    as_str(): "f",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 204,
                                                                                            end: 205,
                                                                                            as_str(): "f",
                                                                                        },
                                                                                    },
                                                                                    arguments: [],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 204,
                                                                                end: 207,
                                                                                as_str(): "f()",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 199,
                                                                end: 208,
                                                                as_str(): "A::A(f())",
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 189,
                                                                                    end: 190,
                                                                                    as_str(): "A",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 192,
                                                                                end: 193,
                                                                                as_str(): "A",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    value: Variable {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 194,
                                                                                end: 195,
                                                                                as_str(): "n",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 195,
                                                                            as_str(): "n",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 189,
                                                                        end: 196,
                                                                        as_str(): "A::A(n)",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                        ),
                                                                                                        start: 211,
                                                                                                        end: 212,
                                                                                                        as_str(): "n",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 211,
                                                                                                end: 212,
                                                                                                as_str(): "n",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 211,
                                                                                        end: 212,
                                                                                        as_str(): "n",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 209,
                                                                                end: 214,
                                                                                as_str(): "{ n }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 209,
                                                                        end: 214,
                                                                        as_str(): "{ n }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 189,
                                                                    end: 214,
                                                                    as_str(): "A::A(n) = A::A(f()) { n }",
                                                                },
                                                            },
                                                            MatchBranch {
                                                                scrutinee: CatchAll {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 220,
                                                                        end: 225,
                                                                        as_str(): "{ 0 }",
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
                                                                                                    0,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 222,
                                                                                                end: 223,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 222,
                                                                                        end: 223,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 220,
                                                                                end: 225,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 220,
                                                                        end: 225,
                                                                        as_str(): "{ 0 }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 220,
                                                                    end: 225,
                                                                    as_str(): "{ 0 }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 182,
                                                    end: 225,
                                                    as_str(): "if let A::A(n) = A::A(f()) { n } else { 0 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 174,
                                    end: 226,
                                    as_str(): "let x = if let A::A(n) = A::A(f()) { n } else { 0 };",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 231,
                                                                end: 237,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 231,
                                                        end: 237,
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
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 240,
                                                                                        end: 242,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 240,
                                                                                        end: 242,
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 240,
                                                                                    end: 242,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 240,
                                                                        end: 242,
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 238,
                                                                                    end: 239,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 238,
                                                                            end: 239,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 243,
                                                                            end: 244,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 238,
                                                            end: 244,
                                                            as_str(): "x == 1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 231,
                                            end: 245,
                                            as_str(): "assert(x == 1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 231,
                                    end: 245,
                                    as_str(): "assert(x == 1)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 256,
                                                    end: 257,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Match(
                                                    MatchExpression {
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
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 277,
                                                                                            end: 278,
                                                                                            as_str(): "A",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 277,
                                                                                        end: 278,
                                                                                        as_str(): "A",
                                                                                    },
                                                                                },
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 280,
                                                                                        end: 281,
                                                                                        as_str(): "B",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 277,
                                                                            end: 281,
                                                                            as_str(): "A::B",
                                                                        },
                                                                    },
                                                                    args: [
                                                                        Expression {
                                                                            kind: FunctionApplication(
                                                                                FunctionApplicationExpression {
                                                                                    call_path_binding: TypeBinding {
                                                                                        inner: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                    ),
                                                                                                    start: 282,
                                                                                                    end: 283,
                                                                                                    as_str(): "g",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 282,
                                                                                            end: 283,
                                                                                            as_str(): "g",
                                                                                        },
                                                                                    },
                                                                                    arguments: [],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 282,
                                                                                end: 285,
                                                                                as_str(): "g()",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 277,
                                                                end: 286,
                                                                as_str(): "A::B(g())",
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 267,
                                                                                    end: 268,
                                                                                    as_str(): "A",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 270,
                                                                                end: 271,
                                                                                as_str(): "B",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    value: Variable {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 272,
                                                                                end: 273,
                                                                                as_str(): "t",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 272,
                                                                            end: 273,
                                                                            as_str(): "t",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 267,
                                                                        end: 274,
                                                                        as_str(): "A::B(t)",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Subfield(
                                                                                                SubfieldExpression {
                                                                                                    prefix: Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 289,
                                                                                                                    end: 290,
                                                                                                                    as_str(): "t",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                            ),
                                                                                                            start: 289,
                                                                                                            end: 290,
                                                                                                            as_str(): "t",
                                                                                                        },
                                                                                                    },
                                                                                                    field_to_access: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                            ),
                                                                                                            start: 291,
                                                                                                            end: 293,
                                                                                                            as_str(): "t1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 289,
                                                                                                end: 293,
                                                                                                as_str(): "t.t1",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 289,
                                                                                        end: 293,
                                                                                        as_str(): "t.t1",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 287,
                                                                                end: 295,
                                                                                as_str(): "{ t.t1 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 287,
                                                                        end: 295,
                                                                        as_str(): "{ t.t1 }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 267,
                                                                    end: 295,
                                                                    as_str(): "A::B(t) = A::B(g()) { t.t1 }",
                                                                },
                                                            },
                                                            MatchBranch {
                                                                scrutinee: CatchAll {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 301,
                                                                        end: 306,
                                                                        as_str(): "{ 0 }",
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
                                                                                                    0,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 303,
                                                                                                end: 304,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 303,
                                                                                        end: 304,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 301,
                                                                                end: 306,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 301,
                                                                        end: 306,
                                                                        as_str(): "{ 0 }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 301,
                                                                    end: 306,
                                                                    as_str(): "{ 0 }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 260,
                                                    end: 306,
                                                    as_str(): "if let A::B(t) = A::B(g()) { t.t1 } else { 0 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 252,
                                    end: 307,
                                    as_str(): "let y = if let A::B(t) = A::B(g()) { t.t1 } else { 0 };",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 312,
                                                                end: 318,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 312,
                                                        end: 318,
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
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 321,
                                                                                    end: 323,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 321,
                                                                        end: 323,
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 319,
                                                                                    end: 320,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 319,
                                                                            end: 320,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 324,
                                                                            end: 326,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 319,
                                                            end: 326,
                                                            as_str(): "y == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 312,
                                            end: 327,
                                            as_str(): "assert(y == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 312,
                                    end: 327,
                                    as_str(): "assert(y == 42)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 338,
                                                    end: 339,
                                                    as_str(): "z",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Match(
                                                    MatchExpression {
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
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 359,
                                                                                            end: 360,
                                                                                            as_str(): "A",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 359,
                                                                                        end: 360,
                                                                                        as_str(): "A",
                                                                                    },
                                                                                },
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 362,
                                                                                        end: 363,
                                                                                        as_str(): "C",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 359,
                                                                            end: 363,
                                                                            as_str(): "A::C",
                                                                        },
                                                                    },
                                                                    args: [
                                                                        Expression {
                                                                            kind: FunctionApplication(
                                                                                FunctionApplicationExpression {
                                                                                    call_path_binding: TypeBinding {
                                                                                        inner: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                    ),
                                                                                                    start: 364,
                                                                                                    end: 365,
                                                                                                    as_str(): "h",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 364,
                                                                                            end: 365,
                                                                                            as_str(): "h",
                                                                                        },
                                                                                    },
                                                                                    arguments: [],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 364,
                                                                                end: 367,
                                                                                as_str(): "h()",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 359,
                                                                end: 368,
                                                                as_str(): "A::C(h())",
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 349,
                                                                                    end: 350,
                                                                                    as_str(): "A",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 352,
                                                                                end: 353,
                                                                                as_str(): "C",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    value: Variable {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 354,
                                                                                end: 355,
                                                                                as_str(): "t",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 354,
                                                                            end: 355,
                                                                            as_str(): "t",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 349,
                                                                        end: 356,
                                                                        as_str(): "A::C(t)",
                                                                    },
                                                                },
                                                                result: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Subfield(
                                                                                                SubfieldExpression {
                                                                                                    prefix: Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 371,
                                                                                                                    end: 372,
                                                                                                                    as_str(): "t",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                            ),
                                                                                                            start: 371,
                                                                                                            end: 372,
                                                                                                            as_str(): "t",
                                                                                                        },
                                                                                                    },
                                                                                                    field_to_access: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                            ),
                                                                                                            start: 373,
                                                                                                            end: 375,
                                                                                                            as_str(): "t2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 371,
                                                                                                end: 375,
                                                                                                as_str(): "t.t2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 371,
                                                                                        end: 375,
                                                                                        as_str(): "t.t2",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 369,
                                                                                end: 377,
                                                                                as_str(): "{ t.t2 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 369,
                                                                        end: 377,
                                                                        as_str(): "{ t.t2 }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 349,
                                                                    end: 377,
                                                                    as_str(): "A::C(t) = A::C(h()) { t.t2 }",
                                                                },
                                                            },
                                                            MatchBranch {
                                                                scrutinee: CatchAll {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 383,
                                                                        end: 388,
                                                                        as_str(): "{ 0 }",
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
                                                                                                    0,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 385,
                                                                                                end: 386,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 385,
                                                                                        end: 386,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 383,
                                                                                end: 388,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 383,
                                                                        end: 388,
                                                                        as_str(): "{ 0 }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 383,
                                                                    end: 388,
                                                                    as_str(): "{ 0 }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 342,
                                                    end: 388,
                                                    as_str(): "if let A::C(t) = A::C(h()) { t.t2 } else { 0 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 334,
                                    end: 389,
                                    as_str(): "let z = if let A::C(t) = A::C(h()) { t.t2 } else { 0 };",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 394,
                                                                end: 400,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 394,
                                                        end: 400,
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
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 403,
                                                                                        end: 405,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 403,
                                                                                        end: 405,
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 403,
                                                                                    end: 405,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 403,
                                                                        end: 405,
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 401,
                                                                                    end: 402,
                                                                                    as_str(): "z",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 401,
                                                                            end: 402,
                                                                            as_str(): "z",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                66,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 406,
                                                                            end: 408,
                                                                            as_str(): "66",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 401,
                                                            end: 408,
                                                            as_str(): "z == 66",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 394,
                                            end: 409,
                                            as_str(): "assert(z == 66)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 394,
                                    end: 409,
                                    as_str(): "assert(z == 66)",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 416,
                                            end: 417,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 416,
                                    end: 417,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 168,
                            end: 419,
                            as_str(): "{\n    let x = if let A::A(n) = A::A(f()) { n } else { 0 };\n    assert(x == 1);\n\n    let y = if let A::B(t) = A::B(g()) { t.t1 } else { 0 };\n    assert(y == 42);\n\n    let z = if let A::C(t) = A::C(h()) { t.t2 } else { 0 };\n    assert(z == 66);\n\n    1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 151,
                        end: 419,
                        as_str(): "fn main() -> u64 {\n    let x = if let A::A(n) = A::A(f()) { n } else { 0 };\n    assert(x == 1);\n\n    let y = if let A::B(t) = A::B(g()) { t.t1 } else { 0 };\n    assert(y == 42);\n\n    let z = if let A::C(t) = A::C(h()) { t.t2 } else { 0 };\n    assert(z == 66);\n\n    1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 164,
                        end: 167,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f2710960,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
            ),
            start: 151,
            end: 419,
            as_str(): "fn main() -> u64 {\n    let x = if let A::A(n) = A::A(f()) { n } else { 0 };\n    assert(x == 1);\n\n    let y = if let A::B(t) = A::B(g()) { t.t1 } else { 0 };\n    assert(y == 42);\n\n    let z = if let A::C(t) = A::C(h()) { t.t2 } else { 0 };\n    assert(z == 66);\n\n    1\n}",
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
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 424,
                            end: 425,
                            as_str(): "f",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 441,
                                            end: 442,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 441,
                                    end: 442,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 435,
                            end: 444,
                            as_str(): "{\n    1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 421,
                        end: 444,
                        as_str(): "fn f() -> u64 {\n    1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 431,
                        end: 434,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f2710960,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
            ),
            start: 421,
            end: 444,
            as_str(): "fn f() -> u64 {\n    1\n}",
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
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 449,
                            end: 450,
                            as_str(): "g",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Struct(
                                            StructExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 465,
                                                                end: 467,
                                                                as_str(): "T1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 465,
                                                        end: 467,
                                                        as_str(): "T1",
                                                    },
                                                },
                                                fields: [
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 470,
                                                                end: 472,
                                                                as_str(): "t1",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 474,
                                                                end: 476,
                                                                as_str(): "42",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 470,
                                                            end: 476,
                                                            as_str(): "t1: 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 465,
                                            end: 478,
                                            as_str(): "T1 { t1: 42 }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 465,
                                    end: 478,
                                    as_str(): "T1 { t1: 42 }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 459,
                            end: 480,
                            as_str(): "{\n    T1 { t1: 42 }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 446,
                        end: 480,
                        as_str(): "fn g() -> T1 {\n    T1 { t1: 42 }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 456,
                                end: 458,
                                as_str(): "T1",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 456,
                        end: 458,
                        as_str(): "T1",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f2710960,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
            ),
            start: 446,
            end: 480,
            as_str(): "fn g() -> T1 {\n    T1 { t1: 42 }\n}",
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
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 485,
                            end: 486,
                            as_str(): "h",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Struct(
                                            StructExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 501,
                                                                end: 503,
                                                                as_str(): "T2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 501,
                                                        end: 503,
                                                        as_str(): "T2",
                                                    },
                                                },
                                                fields: [
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 506,
                                                                end: 508,
                                                                as_str(): "t1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    77,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 510,
                                                                end: 512,
                                                                as_str(): "77",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 506,
                                                            end: 512,
                                                            as_str(): "t1: 77",
                                                        },
                                                    },
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 514,
                                                                end: 516,
                                                                as_str(): "t2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    66,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 518,
                                                                end: 520,
                                                                as_str(): "66",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 514,
                                                            end: 520,
                                                            as_str(): "t2: 66",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 501,
                                            end: 522,
                                            as_str(): "T2 { t1: 77, t2: 66 }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f2710960,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                    ),
                                    start: 501,
                                    end: 522,
                                    as_str(): "T2 { t1: 77, t2: 66 }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 495,
                            end: 524,
                            as_str(): "{\n    T2 { t1: 77, t2: 66 }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 482,
                        end: 524,
                        as_str(): "fn h() -> T2 {\n    T2 { t1: 77, t2: 66 }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb0f2710960,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                ),
                                start: 492,
                                end: 494,
                                as_str(): "T2",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                        ),
                        start: 492,
                        end: 494,
                        as_str(): "T2",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f2710960,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
            ),
            start: 482,
            end: 524,
            as_str(): "fn h() -> T2 {\n    T2 { t1: 77, t2: 66 }\n}",
        },
    },
]
