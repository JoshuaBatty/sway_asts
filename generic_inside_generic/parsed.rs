[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd5d5ae0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                            ),
                            start: 16,
                            end: 24,
                            as_str(): "Generic1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 34,
                                    end: 35,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 37,
                                        end: 38,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 34,
                                end: 38,
                                as_str(): "a: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 37,
                                end: 38,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7249),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd5d5ae0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                        ),
                        start: 9,
                        end: 41,
                        as_str(): "struct Generic1<T> {\n    a: T,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 9,
            end: 41,
            as_str(): "struct Generic1<T> {\n    a: T,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd5d5ae0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                            ),
                            start: 50,
                            end: 58,
                            as_str(): "Generic2",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 68,
                                    end: 69,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 71,
                                        end: 79,
                                        as_str(): "Generic1",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                7250,
                                            ),
                                            initial_type_id: TypeId(
                                                7250,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                ),
                                                start: 80,
                                                end: 81,
                                                as_str(): "T",
                                            },
                                        },
                                    ],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 68,
                                end: 82,
                                as_str(): "b: Generic1<T>",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 71,
                                end: 82,
                                as_str(): "Generic1<T>",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7251),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd5d5ae0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                        ),
                        start: 43,
                        end: 85,
                        as_str(): "struct Generic2<T> {\n    b: Generic1<T>,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 43,
            end: 85,
            as_str(): "struct Generic2<T> {\n    b: Generic1<T>,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd5d5ae0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                            ),
                            start: 92,
                            end: 100,
                            as_str(): "Generic3",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(7252),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 110,
                                    end: 111,
                                    as_str(): "A",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 113,
                                        end: 114,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 113,
                                end: 114,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 110,
                                end: 114,
                                as_str(): "A: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 120,
                                    end: 121,
                                    as_str(): "B",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 123,
                                        end: 124,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 123,
                                end: 124,
                                as_str(): "T",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 120,
                                end: 124,
                                as_str(): "B: T",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fd5d5ae0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                        ),
                        start: 87,
                        end: 126,
                        as_str(): "enum Generic3<T> {\n    A: T,\n    B: T\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 87,
            end: 126,
            as_str(): "enum Generic3<T> {\n    A: T,\n    B: T\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd5d5ae0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                            ),
                            start: 133,
                            end: 141,
                            as_str(): "Generic4",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(7255),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 151,
                                    end: 152,
                                    as_str(): "C",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 154,
                                        end: 162,
                                        as_str(): "Generic3",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                7253,
                                            ),
                                            initial_type_id: TypeId(
                                                7253,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                ),
                                                start: 163,
                                                end: 164,
                                                as_str(): "T",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 154,
                                end: 162,
                                as_str(): "Generic3",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 151,
                                end: 165,
                                as_str(): "C: Generic3<T>",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 171,
                                    end: 172,
                                    as_str(): "D",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 174,
                                        end: 182,
                                        as_str(): "Generic3",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                7254,
                                            ),
                                            initial_type_id: TypeId(
                                                7254,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                ),
                                                start: 183,
                                                end: 184,
                                                as_str(): "T",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 174,
                                end: 182,
                                as_str(): "Generic3",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0fd5d5ae0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                ),
                                start: 171,
                                end: 185,
                                as_str(): "D: Generic3<T>",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fd5d5ae0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                        ),
                        start: 128,
                        end: 187,
                        as_str(): "enum Generic4<T> {\n    C: Generic3<T>,\n    D: Generic3<T>\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 128,
            end: 187,
            as_str(): "enum Generic4<T> {\n    C: Generic3<T>,\n    D: Generic3<T>\n}",
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
                            src (ptr): 0x00007fe0fd5d5ae0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                            ),
                            start: 192,
                            end: 196,
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
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 216,
                                                    end: 217,
                                                    as_str(): "a",
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
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 220,
                                                                        end: 228,
                                                                        as_str(): "Generic1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 220,
                                                                end: 228,
                                                                as_str(): "Generic1",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 239,
                                                                        end: 240,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            7,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 242,
                                                                        end: 246,
                                                                        as_str(): "7u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 239,
                                                                    end: 246,
                                                                    as_str(): "a: 7u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 220,
                                                    end: 252,
                                                    as_str(): "Generic1 {\n        a: 7u64\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 212,
                                    end: 253,
                                    as_str(): "let a = Generic1 {\n        a: 7u64\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 262,
                                                    end: 263,
                                                    as_str(): "b",
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
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 266,
                                                                        end: 274,
                                                                        as_str(): "Generic2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 266,
                                                                end: 274,
                                                                as_str(): "Generic2",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 285,
                                                                        end: 286,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 288,
                                                                                end: 289,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                        ),
                                                                        start: 288,
                                                                        end: 289,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 285,
                                                                    end: 289,
                                                                    as_str(): "b: a",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 266,
                                                    end: 295,
                                                    as_str(): "Generic2 {\n        b: a\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 258,
                                    end: 296,
                                    as_str(): "let b = Generic2 {\n        b: a\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 305,
                                                    end: 306,
                                                    as_str(): "c",
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
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 309,
                                                                                end: 317,
                                                                                as_str(): "Generic3",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 309,
                                                                            end: 317,
                                                                            as_str(): "Generic3",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 319,
                                                                            end: 320,
                                                                            as_str(): "B",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 309,
                                                                end: 320,
                                                                as_str(): "Generic3::B",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 321,
                                                                            end: 322,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 321,
                                                                    end: 322,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 309,
                                                    end: 323,
                                                    as_str(): "Generic3::B(b)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 301,
                                    end: 324,
                                    as_str(): "let c = Generic3::B(b);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 333,
                                                    end: 334,
                                                    as_str(): "d",
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
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 337,
                                                                                end: 345,
                                                                                as_str(): "Generic4",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 337,
                                                                            end: 345,
                                                                            as_str(): "Generic4",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 347,
                                                                            end: 348,
                                                                            as_str(): "C",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 337,
                                                                end: 348,
                                                                as_str(): "Generic4::C",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 349,
                                                                            end: 350,
                                                                            as_str(): "c",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 349,
                                                                    end: 350,
                                                                    as_str(): "c",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 337,
                                                    end: 351,
                                                    as_str(): "Generic4::C(c)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 329,
                                    end: 352,
                                    as_str(): "let d = Generic4::C(c);",
                                },
                            },
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
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 364,
                                                                            end: 365,
                                                                            as_str(): "d",
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
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 364,
                                                                                    end: 365,
                                                                                    as_str(): "d",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 364,
                                                                            end: 365,
                                                                            as_str(): "d",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 358,
                                                            end: 599,
                                                            as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
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
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 364,
                                                                                        end: 365,
                                                                                        as_str(): "d",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 364,
                                                                                end: 365,
                                                                                as_str(): "d",
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
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 376,
                                                                                                    end: 384,
                                                                                                    as_str(): "Generic4",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 386,
                                                                                                end: 387,
                                                                                                as_str(): "C",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: EnumScrutinee {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 401,
                                                                                                        end: 409,
                                                                                                        as_str(): "Generic3",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 411,
                                                                                                    end: 412,
                                                                                                    as_str(): "B",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        value: StructScrutinee {
                                                                                            struct_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 430,
                                                                                                    end: 438,
                                                                                                    as_str(): "Generic2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            fields: [
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 461,
                                                                                                            end: 462,
                                                                                                            as_str(): "b",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: Some(
                                                                                                        StructScrutinee {
                                                                                                            struct_name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 464,
                                                                                                                    end: 472,
                                                                                                                    as_str(): "Generic1",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            fields: [
                                                                                                                Field {
                                                                                                                    field: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 499,
                                                                                                                            end: 500,
                                                                                                                            as_str(): "a",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    scrutinee: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 499,
                                                                                                                        end: 500,
                                                                                                                        as_str(): "a",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 464,
                                                                                                                end: 522,
                                                                                                                as_str(): "Generic1 {\n                        a\n                    }",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 461,
                                                                                                        end: 522,
                                                                                                        as_str(): "b: Generic1 {\n                        a\n                    }",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 540,
                                                                                                as_str(): "Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 401,
                                                                                            end: 554,
                                                                                            as_str(): "Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 376,
                                                                                        end: 564,
                                                                                        as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
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
                                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 570,
                                                                                                                        end: 571,
                                                                                                                        as_str(): "a",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 570,
                                                                                                                end: 571,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 570,
                                                                                                        end: 571,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 568,
                                                                                                end: 573,
                                                                                                as_str(): "{ a }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 568,
                                                                                        end: 573,
                                                                                        as_str(): "{ a }",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 376,
                                                                                    end: 574,
                                                                                    as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: CatchAll {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 583,
                                                                                        end: 584,
                                                                                        as_str(): "_",
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
                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 590,
                                                                                                                end: 591,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 590,
                                                                                                        end: 591,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 588,
                                                                                                end: 593,
                                                                                                as_str(): "{ 0 }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 588,
                                                                                        end: 593,
                                                                                        as_str(): "{ 0 }",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 583,
                                                                                    end: 593,
                                                                                    as_str(): "_ => { 0 }",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 358,
                                                                    end: 599,
                                                                    as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 358,
                                                            end: 599,
                                                            as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 358,
                                                    end: 599,
                                                    as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 358,
                                            end: 599,
                                            as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 358,
                                    end: 599,
                                    as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fd5d5ae0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                            ),
                            start: 206,
                            end: 601,
                            as_str(): "{\n    let a = Generic1 {\n        a: 7u64\n    };\n    let b = Generic2 {\n        b: a\n    };\n    let c = Generic3::B(b);\n    let d = Generic4::C(c);\n\n    match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fd5d5ae0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                        ),
                        start: 189,
                        end: 601,
                        as_str(): "fn main() -> u64 {\n    let a = Generic1 {\n        a: 7u64\n    };\n    let b = Generic2 {\n        b: a\n    };\n    let c = Generic3::B(b);\n    let d = Generic4::C(c);\n\n    match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fd5d5ae0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                        ),
                        start: 202,
                        end: 205,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 189,
            end: 601,
            as_str(): "fn main() -> u64 {\n    let a = Generic1 {\n        a: 7u64\n    };\n    let b = Generic2 {\n        b: a\n    };\n    let c = Generic3::B(b);\n    let d = Generic4::C(c);\n\n    match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }\n}",
        },
    },
]
