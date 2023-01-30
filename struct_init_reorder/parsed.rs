[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
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
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 18,
                            end: 25,
                            as_str(): "logging",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 27,
                            end: 30,
                            as_str(): "log",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 9,
            end: 31,
            as_str(): "use std::logging::log;",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 44,
                            end: 57,
                            as_str(): "MyInnerStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 64,
                                    end: 65,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 64,
                                end: 70,
                                as_str(): "x: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 67,
                                end: 70,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 76,
                                    end: 77,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 76,
                                end: 82,
                                as_str(): "y: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 79,
                                end: 82,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Public,
                    span: Span {
                        src (ptr): 0x00007fe03e568ce0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                        ),
                        start: 33,
                        end: 85,
                        as_str(): "pub struct MyInnerStruct {\n    x: u64,\n    y: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 33,
            end: 85,
            as_str(): "pub struct MyInnerStruct {\n    x: u64,\n    y: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 98,
                            end: 106,
                            as_str(): "MyStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 115,
                                    end: 120,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 122,
                                        end: 135,
                                        as_str(): "MyInnerStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 115,
                                end: 135,
                                as_str(): "value: MyInnerStruct",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 122,
                                end: 135,
                                as_str(): "MyInnerStruct",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Public,
                    span: Span {
                        src (ptr): 0x00007fe03e568ce0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                        ),
                        start: 87,
                        end: 138,
                        as_str(): "pub struct MyStruct {\n      value: MyInnerStruct,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 87,
            end: 138,
            as_str(): "pub struct MyStruct {\n      value: MyInnerStruct,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 149,
                            end: 155,
                            as_str(): "MyEnum",
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
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 162,
                                    end: 164,
                                    as_str(): "V1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                Eight,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 166,
                                end: 168,
                                as_str(): "u8",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 162,
                                end: 168,
                                as_str(): "V1: u8",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 174,
                                    end: 176,
                                    as_str(): "V2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 178,
                                end: 181,
                                as_str(): "u64",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 174,
                                end: 181,
                                as_str(): "V2: u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe03e568ce0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                        ),
                        start: 140,
                        end: 184,
                        as_str(): "pub enum MyEnum {\n    V1: u8,\n    V2: u64,\n}",
                    },
                    visibility: Public,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 140,
            end: 184,
            as_str(): "pub enum MyEnum {\n    V1: u8,\n    V2: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 197,
                            end: 200,
                            as_str(): "Foo",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 207,
                                    end: 209,
                                    as_str(): "f1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 211,
                                        end: 217,
                                        as_str(): "MyEnum",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 207,
                                end: 217,
                                as_str(): "f1: MyEnum",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 211,
                                end: 217,
                                as_str(): "MyEnum",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 223,
                                    end: 225,
                                    as_str(): "f2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 227,
                                        end: 235,
                                        as_str(): "MyStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 223,
                                end: 235,
                                as_str(): "f2: MyStruct",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe03e568ce0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                ),
                                start: 227,
                                end: 235,
                                as_str(): "MyStruct",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Public,
                    span: Span {
                        src (ptr): 0x00007fe03e568ce0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                        ),
                        start: 186,
                        end: 238,
                        as_str(): "pub struct Foo {\n    f1: MyEnum,\n    f2: MyStruct,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 186,
            end: 238,
            as_str(): "pub struct Foo {\n    f1: MyEnum,\n    f2: MyStruct,\n}",
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
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 243,
                            end: 247,
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
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 260,
                                                    end: 262,
                                                    as_str(): "f1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 265,
                                                        end: 271,
                                                        as_str(): "MyEnum",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 265,
                                                    end: 271,
                                                    as_str(): "MyEnum",
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
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 274,
                                                                                end: 280,
                                                                                as_str(): "MyEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 274,
                                                                            end: 280,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 282,
                                                                            end: 284,
                                                                            as_str(): "V1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 274,
                                                                end: 284,
                                                                as_str(): "MyEnum::V1",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 285,
                                                                    end: 288,
                                                                    as_str(): "0u8",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 274,
                                                    end: 289,
                                                    as_str(): "MyEnum::V1(0u8)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 256,
                                    end: 290,
                                    as_str(): "let f1 : MyEnum = MyEnum::V1(0u8);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 299,
                                                    end: 301,
                                                    as_str(): "f2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 304,
                                                        end: 312,
                                                        as_str(): "MyStruct",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 304,
                                                    end: 312,
                                                    as_str(): "MyStruct",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                        ),
                                                                        start: 315,
                                                                        end: 323,
                                                                        as_str(): "MyStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 315,
                                                                end: 323,
                                                                as_str(): "MyStruct",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                        ),
                                                                        start: 326,
                                                                        end: 331,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Struct(
                                                                        StructExpression {
                                                                            call_path_binding: TypeBinding {
                                                                                inner: CallPath {
                                                                                    prefixes: [],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 333,
                                                                                            end: 346,
                                                                                            as_str(): "MyInnerStruct",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                    ),
                                                                                    start: 333,
                                                                                    end: 346,
                                                                                    as_str(): "MyInnerStruct",
                                                                                },
                                                                            },
                                                                            fields: [
                                                                                StructExpressionField {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 349,
                                                                                            end: 350,
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
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 352,
                                                                                            end: 353,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                        ),
                                                                                        start: 349,
                                                                                        end: 353,
                                                                                        as_str(): "x: 0",
                                                                                    },
                                                                                },
                                                                                StructExpressionField {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 355,
                                                                                            end: 356,
                                                                                            as_str(): "y",
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
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 358,
                                                                                            end: 359,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                        ),
                                                                                        start: 355,
                                                                                        end: 359,
                                                                                        as_str(): "y: 0",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                        ),
                                                                        start: 333,
                                                                        end: 361,
                                                                        as_str(): "MyInnerStruct { x: 0, y: 0 }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 326,
                                                                    end: 361,
                                                                    as_str(): "value: MyInnerStruct { x: 0, y: 0 }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 315,
                                                    end: 363,
                                                    as_str(): "MyStruct { value: MyInnerStruct { x: 0, y: 0 } }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 295,
                                    end: 364,
                                    as_str(): "let f2 : MyStruct = MyStruct { value: MyInnerStruct { x: 0, y: 0 } };",
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 452,
                                                                end: 455,
                                                                as_str(): "log",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 452,
                                                        end: 455,
                                                        as_str(): "log",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Struct(
                                                            StructExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 456,
                                                                                end: 459,
                                                                                as_str(): "Foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                        ),
                                                                        start: 456,
                                                                        end: 459,
                                                                        as_str(): "Foo",
                                                                    },
                                                                },
                                                                fields: [
                                                                    StructExpressionField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 470,
                                                                                end: 472,
                                                                                as_str(): "f2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                        ),
                                                                                        start: 470,
                                                                                        end: 472,
                                                                                        as_str(): "f2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 470,
                                                                                end: 472,
                                                                                as_str(): "f2",
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 470,
                                                                            end: 472,
                                                                            as_str(): "f2",
                                                                        },
                                                                    },
                                                                    StructExpressionField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 482,
                                                                                end: 484,
                                                                                as_str(): "f1",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                        ),
                                                                                        start: 482,
                                                                                        end: 484,
                                                                                        as_str(): "f1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 482,
                                                                                end: 484,
                                                                                as_str(): "f1",
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 482,
                                                                            end: 484,
                                                                            as_str(): "f1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 456,
                                                            end: 490,
                                                            as_str(): "Foo {\n        f2,\n        f1\n    }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 452,
                                            end: 491,
                                            as_str(): "log(Foo {\n        f2,\n        f1\n    })",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 452,
                                    end: 491,
                                    as_str(): "log(Foo {\n        f2,\n        f1\n    })",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 250,
                            end: 494,
                            as_str(): "{\n    let f1 : MyEnum = MyEnum::V1(0u8);\n    let f2 : MyStruct = MyStruct { value: MyInnerStruct { x: 0, y: 0 } };\n    // f1 and f2 are instantiated in the wrong order below. that shouldn't matter.\n    log(Foo {\n        f2,\n        f1\n    });\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe03e568ce0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                        ),
                        start: 240,
                        end: 494,
                        as_str(): "fn main() {\n    let f1 : MyEnum = MyEnum::V1(0u8);\n    let f2 : MyStruct = MyStruct { value: MyInnerStruct { x: 0, y: 0 } };\n    // f1 and f2 are instantiated in the wrong order below. that shouldn't matter.\n    log(Foo {\n        f2,\n        f1\n    });\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe03e568ce0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                        ),
                        start: 240,
                        end: 249,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 240,
            end: 494,
            as_str(): "fn main() {\n    let f1 : MyEnum = MyEnum::V1(0u8);\n    let f2 : MyStruct = MyStruct { value: MyInnerStruct { x: 0, y: 0 } };\n    // f1 and f2 are instantiated in the wrong order below. that shouldn't matter.\n    log(Foo {\n        f2,\n        f1\n    });\n}",
        },
    },
]
