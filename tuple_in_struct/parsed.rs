[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 12,
                            end: 15,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 17,
                            end: 23,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 25,
                            end: 31,
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
            src (ptr): 0x00007f8a20324af0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
            ),
            start: 8,
            end: 32,
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
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 41,
                            end: 42,
                            as_str(): "W",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 49,
                                    end: 51,
                                    as_str(): "t5",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 49,
                                end: 56,
                                as_str(): "t5: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 53,
                                end: 56,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 62,
                                    end: 64,
                                    as_str(): "t6",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [
                                    TypeArgument {
                                        type_id: TypeId(
                                            50,
                                        ),
                                        initial_type_id: TypeId(
                                            50,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 67,
                                            end: 69,
                                            as_str(): "u8",
                                        },
                                    },
                                    TypeArgument {
                                        type_id: TypeId(
                                            50,
                                        ),
                                        initial_type_id: TypeId(
                                            50,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 71,
                                            end: 73,
                                            as_str(): "u8",
                                        },
                                    },
                                    TypeArgument {
                                        type_id: TypeId(
                                            31628,
                                        ),
                                        initial_type_id: TypeId(
                                            31628,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 75,
                                            end: 84,
                                            as_str(): "(u64, u8)",
                                        },
                                    },
                                    TypeArgument {
                                        type_id: TypeId(
                                            42,
                                        ),
                                        initial_type_id: TypeId(
                                            42,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 86,
                                            end: 89,
                                            as_str(): "u16",
                                        },
                                    },
                                ],
                            ),
                            span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 62,
                                end: 90,
                                as_str(): "t6: (u8, u8, (u64, u8), u16)",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 66,
                                end: 90,
                                as_str(): "(u8, u8, (u64, u8), u16)",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007f8a20324af0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                        ),
                        start: 34,
                        end: 93,
                        as_str(): "struct W {\n    t5: u64,\n    t6: (u8, u8, (u64, u8), u16),\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a20324af0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
            ),
            start: 34,
            end: 93,
            as_str(): "struct W {\n    t5: u64,\n    t6: (u8, u8, (u64, u8), u16),\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 102,
                            end: 103,
                            as_str(): "T",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 110,
                                    end: 112,
                                    as_str(): "t3",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [
                                    TypeArgument {
                                        type_id: TypeId(
                                            50,
                                        ),
                                        initial_type_id: TypeId(
                                            50,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 115,
                                            end: 117,
                                            as_str(): "u8",
                                        },
                                    },
                                    TypeArgument {
                                        type_id: TypeId(
                                            50,
                                        ),
                                        initial_type_id: TypeId(
                                            50,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 119,
                                            end: 121,
                                            as_str(): "u8",
                                        },
                                    },
                                ],
                            ),
                            span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 110,
                                end: 122,
                                as_str(): "t3: (u8, u8)",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 114,
                                end: 122,
                                as_str(): "(u8, u8)",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 128,
                                    end: 130,
                                    as_str(): "t4",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                Sixteen,
                            ),
                            span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 128,
                                end: 135,
                                as_str(): "t4: u16",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 132,
                                end: 135,
                                as_str(): "u16",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007f8a20324af0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                        ),
                        start: 95,
                        end: 137,
                        as_str(): "struct T {\n    t3: (u8, u8),\n    t4: u16\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a20324af0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
            ),
            start: 95,
            end: 137,
            as_str(): "struct T {\n    t3: (u8, u8),\n    t4: u16\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 146,
                            end: 147,
                            as_str(): "S",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 154,
                                    end: 156,
                                    as_str(): "t0",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 159,
                                        as_str(): "W",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 154,
                                end: 159,
                                as_str(): "t0: W",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 158,
                                end: 159,
                                as_str(): "W",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 165,
                                    end: 167,
                                    as_str(): "t1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [
                                    TypeArgument {
                                        type_id: TypeId(
                                            21,
                                        ),
                                        initial_type_id: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 170,
                                            end: 173,
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
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 175,
                                            end: 178,
                                            as_str(): "u64",
                                        },
                                    },
                                ],
                            ),
                            span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 165,
                                end: 179,
                                as_str(): "t1: (u64, u64)",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 169,
                                end: 179,
                                as_str(): "(u64, u64)",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 185,
                                    end: 187,
                                    as_str(): "t2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 189,
                                        end: 190,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 185,
                                end: 190,
                                as_str(): "t2: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 189,
                                end: 190,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007f8a20324af0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                        ),
                        start: 139,
                        end: 193,
                        as_str(): "struct S {\n    t0: W,\n    t1: (u64, u64),\n    t2: T,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a20324af0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
            ),
            start: 139,
            end: 193,
            as_str(): "struct S {\n    t0: W,\n    t1: (u64, u64),\n    t2: T,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 202,
                            end: 203,
                            as_str(): "U",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 210,
                                    end: 211,
                                    as_str(): "u",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 210,
                                end: 216,
                                as_str(): "u: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a20324af0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                ),
                                start: 213,
                                end: 216,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007f8a20324af0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                        ),
                        start: 195,
                        end: 218,
                        as_str(): "struct U {\n    u: u64\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a20324af0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
            ),
            start: 195,
            end: 218,
            as_str(): "struct U {\n    u: u64\n}",
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
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 223,
                            end: 227,
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
                                                    src (ptr): 0x00007f8a20324af0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                    ),
                                                    start: 248,
                                                    end: 249,
                                                    as_str(): "s",
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 252,
                                                                        end: 253,
                                                                        as_str(): "S",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 252,
                                                                end: 253,
                                                                as_str(): "S",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 264,
                                                                        end: 266,
                                                                        as_str(): "t0",
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
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 268,
                                                                                            end: 269,
                                                                                            as_str(): "W",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 268,
                                                                                    end: 269,
                                                                                    as_str(): "W",
                                                                                },
                                                                            },
                                                                            fields: [
                                                                                StructExpressionField {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 284,
                                                                                            end: 286,
                                                                                            as_str(): "t5",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value: Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                5,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 288,
                                                                                            end: 289,
                                                                                            as_str(): "5",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 284,
                                                                                        end: 289,
                                                                                        as_str(): "t5: 5",
                                                                                    },
                                                                                },
                                                                                StructExpressionField {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 303,
                                                                                            end: 305,
                                                                                            as_str(): "t6",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value: Expression {
                                                                                        kind: Tuple(
                                                                                            [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            6,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 308,
                                                                                                        end: 309,
                                                                                                        as_str(): "6",
                                                                                                    },
                                                                                                },
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            7,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 311,
                                                                                                        end: 312,
                                                                                                        as_str(): "7",
                                                                                                    },
                                                                                                },
                                                                                                Expression {
                                                                                                    kind: Tuple(
                                                                                                        [
                                                                                                            Expression {
                                                                                                                kind: Literal(
                                                                                                                    Numeric(
                                                                                                                        8,
                                                                                                                    ),
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 315,
                                                                                                                    end: 316,
                                                                                                                    as_str(): "8",
                                                                                                                },
                                                                                                            },
                                                                                                            Expression {
                                                                                                                kind: Literal(
                                                                                                                    Numeric(
                                                                                                                        9,
                                                                                                                    ),
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 318,
                                                                                                                    end: 319,
                                                                                                                    as_str(): "9",
                                                                                                                },
                                                                                                            },
                                                                                                        ],
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 314,
                                                                                                        end: 320,
                                                                                                        as_str(): "(8, 9)",
                                                                                                    },
                                                                                                },
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            10,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 322,
                                                                                                        end: 324,
                                                                                                        as_str(): "10",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 307,
                                                                                            end: 325,
                                                                                            as_str(): "(6, 7, (8, 9), 10)",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 303,
                                                                                        end: 325,
                                                                                        as_str(): "t6: (6, 7, (8, 9), 10)",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 268,
                                                                        end: 335,
                                                                        as_str(): "W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a20324af0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 264,
                                                                    end: 335,
                                                                    as_str(): "t0: W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        }",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 345,
                                                                        end: 347,
                                                                        as_str(): "t1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Tuple(
                                                                        [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 350,
                                                                                    end: 351,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 353,
                                                                                    end: 354,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        ],
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 349,
                                                                        end: 355,
                                                                        as_str(): "(0, 1)",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a20324af0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 345,
                                                                    end: 355,
                                                                    as_str(): "t1: (0, 1)",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 365,
                                                                        end: 367,
                                                                        as_str(): "t2",
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
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 369,
                                                                                            end: 370,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 369,
                                                                                    end: 370,
                                                                                    as_str(): "T",
                                                                                },
                                                                            },
                                                                            fields: [
                                                                                StructExpressionField {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 385,
                                                                                            end: 387,
                                                                                            as_str(): "t3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value: Expression {
                                                                                        kind: Tuple(
                                                                                            [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            2,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 390,
                                                                                                        end: 391,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                },
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            3,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 393,
                                                                                                        end: 394,
                                                                                                        as_str(): "3",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 389,
                                                                                            end: 395,
                                                                                            as_str(): "(2, 3)",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 385,
                                                                                        end: 395,
                                                                                        as_str(): "t3: (2, 3)",
                                                                                    },
                                                                                },
                                                                                StructExpressionField {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 409,
                                                                                            end: 411,
                                                                                            as_str(): "t4",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value: Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                4,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 413,
                                                                                            end: 414,
                                                                                            as_str(): "4",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 409,
                                                                                        end: 414,
                                                                                        as_str(): "t4: 4",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 369,
                                                                        end: 424,
                                                                        as_str(): "T {\n            t3: (2, 3),\n            t4: 4\n        }",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a20324af0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 365,
                                                                    end: 424,
                                                                    as_str(): "t2: T {\n            t3: (2, 3),\n            t4: 4\n        }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a20324af0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                    ),
                                                    start: 252,
                                                    end: 430,
                                                    as_str(): "S {\n        t0: W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        },\n        t1: (0, 1),\n        t2: T {\n            t3: (2, 3),\n            t4: 4\n        }\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 244,
                                    end: 431,
                                    as_str(): "let s = S {\n        t0: W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        },\n        t1: (0, 1),\n        t2: T {\n            t3: (2, 3),\n            t4: 4\n        }\n    };",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 441,
                                                                end: 447,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 441,
                                                        end: 447,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 457,
                                                                                        end: 459,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 457,
                                                                                        end: 459,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 457,
                                                                                    end: 459,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 457,
                                                                        end: 459,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 449,
                                                                                                            end: 450,
                                                                                                            as_str(): "s",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 449,
                                                                                                    end: 450,
                                                                                                    as_str(): "s",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 451,
                                                                                                    end: 453,
                                                                                                    as_str(): "t1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 449,
                                                                                        end: 453,
                                                                                        as_str(): "s.t1",
                                                                                    },
                                                                                },
                                                                                index: 0,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 455,
                                                                                    end: 456,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 448,
                                                                            end: 456,
                                                                            as_str(): "(s.t1).0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 460,
                                                                            end: 461,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 448,
                                                            end: 461,
                                                            as_str(): "(s.t1).0 == 0",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 441,
                                            end: 462,
                                            as_str(): "assert((s.t1).0 == 0)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 441,
                                    end: 462,
                                    as_str(): "assert((s.t1).0 == 0)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 468,
                                                                end: 474,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 468,
                                                        end: 474,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 484,
                                                                                        end: 486,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 484,
                                                                                        end: 486,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 484,
                                                                                    end: 486,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 484,
                                                                        end: 486,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 476,
                                                                                                            end: 477,
                                                                                                            as_str(): "s",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 476,
                                                                                                    end: 477,
                                                                                                    as_str(): "s",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 478,
                                                                                                    end: 480,
                                                                                                    as_str(): "t1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 476,
                                                                                        end: 480,
                                                                                        as_str(): "s.t1",
                                                                                    },
                                                                                },
                                                                                index: 1,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 482,
                                                                                    end: 483,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 475,
                                                                            end: 483,
                                                                            as_str(): "(s.t1).1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 487,
                                                                            end: 488,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 475,
                                                            end: 488,
                                                            as_str(): "(s.t1).1 == 1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 468,
                                            end: 489,
                                            as_str(): "assert((s.t1).1 == 1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 468,
                                    end: 489,
                                    as_str(): "assert((s.t1).1 == 1)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 495,
                                                                end: 501,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 495,
                                                        end: 501,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 514,
                                                                                        end: 516,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 514,
                                                                                        end: 516,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 514,
                                                                                    end: 516,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 514,
                                                                        end: 516,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Subfield(
                                                                                                    SubfieldExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 503,
                                                                                                                        end: 504,
                                                                                                                        as_str(): "s",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 503,
                                                                                                                end: 504,
                                                                                                                as_str(): "s",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 505,
                                                                                                                end: 507,
                                                                                                                as_str(): "t2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 503,
                                                                                                    end: 507,
                                                                                                    as_str(): "s.t2",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 508,
                                                                                                    end: 510,
                                                                                                    as_str(): "t3",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 503,
                                                                                        end: 510,
                                                                                        as_str(): "s.t2.t3",
                                                                                    },
                                                                                },
                                                                                index: 0,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 512,
                                                                                    end: 513,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 502,
                                                                            end: 513,
                                                                            as_str(): "(s.t2.t3).0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 517,
                                                                            end: 518,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 502,
                                                            end: 518,
                                                            as_str(): "(s.t2.t3).0 == 2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 495,
                                            end: 519,
                                            as_str(): "assert((s.t2.t3).0 == 2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 495,
                                    end: 519,
                                    as_str(): "assert((s.t2.t3).0 == 2)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 525,
                                                                end: 531,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 525,
                                                        end: 531,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 544,
                                                                                        end: 546,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 544,
                                                                                        end: 546,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 544,
                                                                                    end: 546,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 544,
                                                                        end: 546,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Subfield(
                                                                                                    SubfieldExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 533,
                                                                                                                        end: 534,
                                                                                                                        as_str(): "s",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 533,
                                                                                                                end: 534,
                                                                                                                as_str(): "s",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 535,
                                                                                                                end: 537,
                                                                                                                as_str(): "t2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 533,
                                                                                                    end: 537,
                                                                                                    as_str(): "s.t2",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 538,
                                                                                                    end: 540,
                                                                                                    as_str(): "t3",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 533,
                                                                                        end: 540,
                                                                                        as_str(): "s.t2.t3",
                                                                                    },
                                                                                },
                                                                                index: 1,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 542,
                                                                                    end: 543,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 532,
                                                                            end: 543,
                                                                            as_str(): "(s.t2.t3).1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 547,
                                                                            end: 548,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 532,
                                                            end: 548,
                                                            as_str(): "(s.t2.t3).1 == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 525,
                                            end: 549,
                                            as_str(): "assert((s.t2.t3).1 == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 525,
                                    end: 549,
                                    as_str(): "assert((s.t2.t3).1 == 3)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 555,
                                                                end: 561,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 555,
                                                        end: 561,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 570,
                                                                                        end: 572,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 570,
                                                                                        end: 572,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 570,
                                                                                    end: 572,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 570,
                                                                        end: 572,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 562,
                                                                                                            end: 563,
                                                                                                            as_str(): "s",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 562,
                                                                                                    end: 563,
                                                                                                    as_str(): "s",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 564,
                                                                                                    end: 566,
                                                                                                    as_str(): "t2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 562,
                                                                                        end: 566,
                                                                                        as_str(): "s.t2",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 567,
                                                                                        end: 569,
                                                                                        as_str(): "t4",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 562,
                                                                            end: 569,
                                                                            as_str(): "s.t2.t4",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 573,
                                                                            end: 574,
                                                                            as_str(): "4",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 562,
                                                            end: 574,
                                                            as_str(): "s.t2.t4 == 4",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 555,
                                            end: 575,
                                            as_str(): "assert(s.t2.t4 == 4)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 555,
                                    end: 575,
                                    as_str(): "assert(s.t2.t4 == 4)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 581,
                                                                end: 587,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 581,
                                                        end: 587,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 600,
                                                                                        end: 602,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 600,
                                                                                        end: 602,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 600,
                                                                                    end: 602,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 600,
                                                                        end: 602,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 590,
                                                                                                            end: 591,
                                                                                                            as_str(): "s",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 590,
                                                                                                    end: 591,
                                                                                                    as_str(): "s",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 592,
                                                                                                    end: 594,
                                                                                                    as_str(): "t0",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 590,
                                                                                        end: 594,
                                                                                        as_str(): "s.t0",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 596,
                                                                                        end: 598,
                                                                                        as_str(): "t5",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 589,
                                                                            end: 598,
                                                                            as_str(): "(s.t0).t5",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 603,
                                                                            end: 604,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 588,
                                                            end: 604,
                                                            as_str(): "((s.t0).t5) == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 581,
                                            end: 605,
                                            as_str(): "assert(((s.t0).t5) == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 581,
                                    end: 605,
                                    as_str(): "assert(((s.t0).t5) == 5)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 611,
                                                                end: 617,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 611,
                                                        end: 617,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 632,
                                                                                        end: 634,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 632,
                                                                                        end: 634,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 632,
                                                                                    end: 634,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 632,
                                                                        end: 634,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Subfield(
                                                                                                    SubfieldExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 620,
                                                                                                                        end: 621,
                                                                                                                        as_str(): "s",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 620,
                                                                                                                end: 621,
                                                                                                                as_str(): "s",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 622,
                                                                                                                end: 624,
                                                                                                                as_str(): "t0",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 620,
                                                                                                    end: 624,
                                                                                                    as_str(): "s.t0",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 626,
                                                                                                    end: 628,
                                                                                                    as_str(): "t6",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 619,
                                                                                        end: 628,
                                                                                        as_str(): "(s.t0).t6",
                                                                                    },
                                                                                },
                                                                                index: 0,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 630,
                                                                                    end: 631,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 618,
                                                                            end: 631,
                                                                            as_str(): "((s.t0).t6).0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                6,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 635,
                                                                            end: 636,
                                                                            as_str(): "6",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 618,
                                                            end: 636,
                                                            as_str(): "((s.t0).t6).0 == 6",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 611,
                                            end: 637,
                                            as_str(): "assert(((s.t0).t6).0 == 6)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 611,
                                    end: 637,
                                    as_str(): "assert(((s.t0).t6).0 == 6)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 643,
                                                                end: 649,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 643,
                                                        end: 649,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 664,
                                                                                        end: 666,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 664,
                                                                                        end: 666,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 664,
                                                                                    end: 666,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 664,
                                                                        end: 666,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Subfield(
                                                                                                    SubfieldExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 652,
                                                                                                                        end: 653,
                                                                                                                        as_str(): "s",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 652,
                                                                                                                end: 653,
                                                                                                                as_str(): "s",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 654,
                                                                                                                end: 656,
                                                                                                                as_str(): "t0",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 652,
                                                                                                    end: 656,
                                                                                                    as_str(): "s.t0",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 658,
                                                                                                    end: 660,
                                                                                                    as_str(): "t6",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 651,
                                                                                        end: 660,
                                                                                        as_str(): "(s.t0).t6",
                                                                                    },
                                                                                },
                                                                                index: 1,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 662,
                                                                                    end: 663,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 650,
                                                                            end: 663,
                                                                            as_str(): "((s.t0).t6).1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 667,
                                                                            end: 668,
                                                                            as_str(): "7",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 650,
                                                            end: 668,
                                                            as_str(): "((s.t0).t6).1 == 7",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 643,
                                            end: 669,
                                            as_str(): "assert(((s.t0).t6).1 == 7)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 643,
                                    end: 669,
                                    as_str(): "assert(((s.t0).t6).1 == 7)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 675,
                                                                end: 681,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 675,
                                                        end: 681,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 700,
                                                                                        end: 702,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 700,
                                                                                        end: 702,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 700,
                                                                                    end: 702,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 700,
                                                                        end: 702,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: TupleIndex(
                                                                                        TupleIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Subfield(
                                                                                                    SubfieldExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Subfield(
                                                                                                                SubfieldExpression {
                                                                                                                    prefix: Expression {
                                                                                                                        kind: Variable(
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 685,
                                                                                                                                    end: 686,
                                                                                                                                    as_str(): "s",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 685,
                                                                                                                            end: 686,
                                                                                                                            as_str(): "s",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    field_to_access: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 687,
                                                                                                                            end: 689,
                                                                                                                            as_str(): "t0",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 685,
                                                                                                                end: 689,
                                                                                                                as_str(): "s.t0",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 691,
                                                                                                                end: 693,
                                                                                                                as_str(): "t6",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 684,
                                                                                                    end: 693,
                                                                                                    as_str(): "(s.t0).t6",
                                                                                                },
                                                                                            },
                                                                                            index: 2,
                                                                                            index_span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 695,
                                                                                                end: 696,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 683,
                                                                                        end: 696,
                                                                                        as_str(): "((s.t0).t6).2",
                                                                                    },
                                                                                },
                                                                                index: 0,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 698,
                                                                                    end: 699,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 682,
                                                                            end: 699,
                                                                            as_str(): "(((s.t0).t6).2).0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                8,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 703,
                                                                            end: 704,
                                                                            as_str(): "8",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 682,
                                                            end: 704,
                                                            as_str(): "(((s.t0).t6).2).0 == 8",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 675,
                                            end: 705,
                                            as_str(): "assert((((s.t0).t6).2).0 == 8)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 675,
                                    end: 705,
                                    as_str(): "assert((((s.t0).t6).2).0 == 8)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 711,
                                                                end: 717,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 711,
                                                        end: 717,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 736,
                                                                                        end: 738,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 736,
                                                                                        end: 738,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 736,
                                                                                    end: 738,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 736,
                                                                        end: 738,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: TupleIndex(
                                                                                        TupleIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Subfield(
                                                                                                    SubfieldExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Subfield(
                                                                                                                SubfieldExpression {
                                                                                                                    prefix: Expression {
                                                                                                                        kind: Variable(
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 721,
                                                                                                                                    end: 722,
                                                                                                                                    as_str(): "s",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 721,
                                                                                                                            end: 722,
                                                                                                                            as_str(): "s",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    field_to_access: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 723,
                                                                                                                            end: 725,
                                                                                                                            as_str(): "t0",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 721,
                                                                                                                end: 725,
                                                                                                                as_str(): "s.t0",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 727,
                                                                                                                end: 729,
                                                                                                                as_str(): "t6",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 720,
                                                                                                    end: 729,
                                                                                                    as_str(): "(s.t0).t6",
                                                                                                },
                                                                                            },
                                                                                            index: 2,
                                                                                            index_span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 731,
                                                                                                end: 732,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 719,
                                                                                        end: 732,
                                                                                        as_str(): "((s.t0).t6).2",
                                                                                    },
                                                                                },
                                                                                index: 1,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 734,
                                                                                    end: 735,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 718,
                                                                            end: 735,
                                                                            as_str(): "(((s.t0).t6).2).1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                9,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 739,
                                                                            end: 740,
                                                                            as_str(): "9",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 718,
                                                            end: 740,
                                                            as_str(): "(((s.t0).t6).2).1 == 9",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 711,
                                            end: 741,
                                            as_str(): "assert((((s.t0).t6).2).1 == 9)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 711,
                                    end: 741,
                                    as_str(): "assert((((s.t0).t6).2).1 == 9)",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 747,
                                                                end: 753,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 747,
                                                        end: 753,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 768,
                                                                                        end: 770,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 768,
                                                                                        end: 770,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 768,
                                                                                    end: 770,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 768,
                                                                        end: 770,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Subfield(
                                                                                                    SubfieldExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 756,
                                                                                                                        end: 757,
                                                                                                                        as_str(): "s",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 756,
                                                                                                                end: 757,
                                                                                                                as_str(): "s",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 758,
                                                                                                                end: 760,
                                                                                                                as_str(): "t0",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 756,
                                                                                                    end: 760,
                                                                                                    as_str(): "s.t0",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 762,
                                                                                                    end: 764,
                                                                                                    as_str(): "t6",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 755,
                                                                                        end: 764,
                                                                                        as_str(): "(s.t0).t6",
                                                                                    },
                                                                                },
                                                                                index: 3,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 766,
                                                                                    end: 767,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 754,
                                                                            end: 767,
                                                                            as_str(): "((s.t0).t6).3",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                10,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 771,
                                                                            end: 773,
                                                                            as_str(): "10",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 754,
                                                            end: 773,
                                                            as_str(): "((s.t0).t6).3 == 10",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 747,
                                            end: 774,
                                            as_str(): "assert(((s.t0).t6).3 == 10)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 747,
                                    end: 774,
                                    as_str(): "assert(((s.t0).t6).3 == 10)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a20324af0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                    ),
                                                    start: 785,
                                                    end: 786,
                                                    as_str(): "u",
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 789,
                                                                        end: 790,
                                                                        as_str(): "U",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 789,
                                                                end: 790,
                                                                as_str(): "U",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 801,
                                                                        end: 802,
                                                                        as_str(): "u",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            22,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 804,
                                                                        end: 806,
                                                                        as_str(): "22",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a20324af0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 801,
                                                                    end: 806,
                                                                    as_str(): "u: 22",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a20324af0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                    ),
                                                    start: 789,
                                                    end: 813,
                                                    as_str(): "U {\n        u: 22 \n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 781,
                                    end: 814,
                                    as_str(): "let u = U {\n        u: 22 \n    };",
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 819,
                                                                end: 825,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 819,
                                                        end: 825,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 830,
                                                                                        end: 832,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 830,
                                                                                        end: 832,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 830,
                                                                                    end: 832,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 830,
                                                                        end: 832,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 826,
                                                                                                end: 827,
                                                                                                as_str(): "u",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 826,
                                                                                        end: 827,
                                                                                        as_str(): "u",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 828,
                                                                                        end: 829,
                                                                                        as_str(): "u",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 826,
                                                                            end: 829,
                                                                            as_str(): "u.u",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                22,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 833,
                                                                            end: 835,
                                                                            as_str(): "22",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 826,
                                                            end: 835,
                                                            as_str(): "u.u == 22",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 819,
                                            end: 836,
                                            as_str(): "assert(u.u == 22)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 819,
                                    end: 836,
                                    as_str(): "assert(u.u == 22)",
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
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 843,
                                            end: 847,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a20324af0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                    ),
                                    start: 843,
                                    end: 847,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 238,
                            end: 849,
                            as_str(): "{\n    let s = S {\n        t0: W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        },\n        t1: (0, 1),\n        t2: T {\n            t3: (2, 3),\n            t4: 4\n        }\n    };\n    \n    assert((s.t1).0 == 0);\n    assert((s.t1).1 == 1);\n    assert((s.t2.t3).0 == 2);\n    assert((s.t2.t3).1 == 3);\n    assert(s.t2.t4 == 4);\n    assert(((s.t0).t5) == 5);\n    assert(((s.t0).t6).0 == 6);\n    assert(((s.t0).t6).1 == 7);\n    assert((((s.t0).t6).2).0 == 8);\n    assert((((s.t0).t6).2).1 == 9);\n    assert(((s.t0).t6).3 == 10);\n\n    let u = U {\n        u: 22 \n    };\n    assert(u.u == 22);\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a20324af0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                        ),
                        start: 220,
                        end: 849,
                        as_str(): "fn main() -> bool {\n    let s = S {\n        t0: W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        },\n        t1: (0, 1),\n        t2: T {\n            t3: (2, 3),\n            t4: 4\n        }\n    };\n    \n    assert((s.t1).0 == 0);\n    assert((s.t1).1 == 1);\n    assert((s.t2.t3).0 == 2);\n    assert((s.t2.t3).1 == 3);\n    assert(s.t2.t4 == 4);\n    assert(((s.t0).t5) == 5);\n    assert(((s.t0).t6).0 == 6);\n    assert(((s.t0).t6).1 == 7);\n    assert((((s.t0).t6).2).0 == 8);\n    assert((((s.t0).t6).2).1 == 9);\n    assert(((s.t0).t6).3 == 10);\n\n    let u = U {\n        u: 22 \n    };\n    assert(u.u == 22);\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a20324af0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                        ),
                        start: 233,
                        end: 237,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a20324af0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
            ),
            start: 220,
            end: 849,
            as_str(): "fn main() -> bool {\n    let s = S {\n        t0: W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        },\n        t1: (0, 1),\n        t2: T {\n            t3: (2, 3),\n            t4: 4\n        }\n    };\n    \n    assert((s.t1).0 == 0);\n    assert((s.t1).1 == 1);\n    assert((s.t2.t3).0 == 2);\n    assert((s.t2.t3).1 == 3);\n    assert(s.t2.t4 == 4);\n    assert(((s.t0).t5) == 5);\n    assert(((s.t0).t6).0 == 6);\n    assert(((s.t0).t6).1 == 7);\n    assert((((s.t0).t6).2).0 == 8);\n    assert((((s.t0).t6).2).1 == 9);\n    assert(((s.t0).t6).3 == 10);\n\n    let u = U {\n        u: 22 \n    };\n    assert(u.u == 22);\n\n    true\n}",
        },
    },
]
