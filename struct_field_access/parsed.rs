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
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 94,
                            end: 98,
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
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 126,
                                                    as_str(): "data",
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
                                                                        src (ptr): 0x00007fe04b55c700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                        ),
                                                                        start: 129,
                                                                        end: 133,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 133,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b55c700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                        ),
                                                                        start: 144,
                                                                        end: 157,
                                                                        as_str(): "uselessnumber",
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
                                                                        src (ptr): 0x00007fe04b55c700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                        ),
                                                                        start: 159,
                                                                        end: 161,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04b55c700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                    ),
                                                                    start: 144,
                                                                    end: 161,
                                                                    as_str(): "uselessnumber: 42",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 129,
                                                    end: 168,
                                                    as_str(): "Data {\n        uselessnumber: 42,\n    }",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 169,
                                    as_str(): "let mut data = Data {\n        uselessnumber: 42,\n    };",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Reassignment(
                                            ReassignmentExpression {
                                                lhs: VariableExpression(
                                                    Expression {
                                                        kind: Subfield(
                                                            SubfieldExpression {
                                                                prefix: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04b55c700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                ),
                                                                                start: 174,
                                                                                end: 178,
                                                                                as_str(): "data",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b55c700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                        ),
                                                                        start: 174,
                                                                        end: 178,
                                                                        as_str(): "data",
                                                                    },
                                                                },
                                                                field_to_access: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b55c700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                        ),
                                                                        start: 179,
                                                                        end: 192,
                                                                        as_str(): "uselessnumber",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 174,
                                                            end: 192,
                                                            as_str(): "data.uselessnumber",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Numeric(
                                                            43,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b55c700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                        ),
                                                        start: 195,
                                                        end: 197,
                                                        as_str(): "43",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 174,
                                            end: 197,
                                            as_str(): "data.uselessnumber = 43",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 174,
                                    end: 197,
                                    as_str(): "data.uselessnumber = 43",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 208,
                                                    end: 213,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: FunctionApplication(
                                                                FunctionApplicationExpression {
                                                                    call_path_binding: TypeBinding {
                                                                        inner: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04b55c700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                    ),
                                                                                    start: 216,
                                                                                    end: 226,
                                                                                    as_str(): "ret_struct",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04b55c700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                            ),
                                                                            start: 216,
                                                                            end: 226,
                                                                            as_str(): "ret_struct",
                                                                        },
                                                                    },
                                                                    arguments: [],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 216,
                                                                end: 228,
                                                                as_str(): "ret_struct()",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 229,
                                                                end: 242,
                                                                as_str(): "uselessnumber",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 216,
                                                    end: 242,
                                                    as_str(): "ret_struct().uselessnumber",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 204,
                                    end: 243,
                                    as_str(): "let other = ret_struct().uselessnumber;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b55c700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                        ),
                                                                        start: 256,
                                                                        end: 260,
                                                                        as_str(): "data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 256,
                                                                end: 260,
                                                                as_str(): "data",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 261,
                                                                end: 274,
                                                                as_str(): "uselessnumber",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 256,
                                                    end: 274,
                                                    as_str(): "data.uselessnumber",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 249,
                                            end: 274,
                                            as_str(): "return data.uselessnumber",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 249,
                                    end: 274,
                                    as_str(): "return data.uselessnumber",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 108,
                            end: 277,
                            as_str(): "{\n    let mut data = Data {\n        uselessnumber: 42,\n    };\n    data.uselessnumber = 43;\n\n    let other = ret_struct().uselessnumber;\n\n    return data.uselessnumber;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe04b55c700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                        ),
                        start: 91,
                        end: 277,
                        as_str(): "fn main() -> u64 {\n    let mut data = Data {\n        uselessnumber: 42,\n    };\n    data.uselessnumber = 43;\n\n    let other = ret_struct().uselessnumber;\n\n    return data.uselessnumber;\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe04b55c700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                        ),
                        start: 104,
                        end: 107,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04b55c700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
            ),
            start: 91,
            end: 277,
            as_str(): "fn main() -> u64 {\n    let mut data = Data {\n        uselessnumber: 42,\n    };\n    data.uselessnumber = 43;\n\n    let other = ret_struct().uselessnumber;\n\n    return data.uselessnumber;\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 286,
                            end: 290,
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
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 297,
                                    end: 310,
                                    as_str(): "uselessnumber",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04b55c700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                ),
                                start: 297,
                                end: 315,
                                as_str(): "uselessnumber: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04b55c700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                ),
                                start: 312,
                                end: 315,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe04b55c700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                        ),
                        start: 279,
                        end: 318,
                        as_str(): "struct Data {\n    uselessnumber: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04b55c700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
            ),
            start: 279,
            end: 318,
            as_str(): "struct Data {\n    uselessnumber: u64,\n}",
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
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 323,
                            end: 333,
                            as_str(): "ret_struct",
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
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 350,
                                                                end: 354,
                                                                as_str(): "Data",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b55c700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                        ),
                                                        start: 350,
                                                        end: 354,
                                                        as_str(): "Data",
                                                    },
                                                },
                                                fields: [
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 365,
                                                                end: 378,
                                                                as_str(): "uselessnumber",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    44,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 380,
                                                                end: 382,
                                                                as_str(): "44",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 365,
                                                            end: 382,
                                                            as_str(): "uselessnumber: 44",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 350,
                                            end: 389,
                                            as_str(): "Data {\n        uselessnumber: 44,\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b55c700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                    ),
                                    start: 350,
                                    end: 389,
                                    as_str(): "Data {\n        uselessnumber: 44,\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 344,
                            end: 391,
                            as_str(): "{\n    Data {\n        uselessnumber: 44,\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe04b55c700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                        ),
                        start: 320,
                        end: 391,
                        as_str(): "fn ret_struct() -> Data {\n    Data {\n        uselessnumber: 44,\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe04b55c700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                ),
                                start: 339,
                                end: 343,
                                as_str(): "Data",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe04b55c700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                        ),
                        start: 339,
                        end: 343,
                        as_str(): "Data",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04b55c700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
            ),
            start: 320,
            end: 391,
            as_str(): "fn ret_struct() -> Data {\n    Data {\n        uselessnumber: 44,\n    }\n}",
        },
    },
]
