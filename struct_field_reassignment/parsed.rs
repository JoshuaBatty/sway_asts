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
                            src (ptr): 0x00007fe056c4b800,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                            ),
                            start: 57,
                            end: 61,
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
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 85,
                                                    end: 89,
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
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 92,
                                                                        end: 96,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 96,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 107,
                                                                        end: 112,
                                                                        as_str(): "value",
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
                                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 114,
                                                                                                    end: 128,
                                                                                                    as_str(): "NumberOrString",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 114,
                                                                                                end: 128,
                                                                                                as_str(): "NumberOrString",
                                                                                            },
                                                                                        },
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 130,
                                                                                                end: 136,
                                                                                                as_str(): "Number",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 114,
                                                                                    end: 136,
                                                                                    as_str(): "NumberOrString::Number",
                                                                                },
                                                                            },
                                                                            args: [
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            20,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056c4b800,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 137,
                                                                                        end: 139,
                                                                                        as_str(): "20",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 114,
                                                                        end: 140,
                                                                        as_str(): "NumberOrString::Number(20)",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 107,
                                                                    end: 140,
                                                                    as_str(): "value: NumberOrString::Number(20)",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 150,
                                                                        end: 157,
                                                                        as_str(): "address",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            15,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 159,
                                                                        end: 169,
                                                                        as_str(): "0b00001111",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 150,
                                                                    end: 169,
                                                                    as_str(): "address: 0b00001111",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 92,
                                                    end: 176,
                                                    as_str(): "Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    }",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 77,
                                    end: 177,
                                    as_str(): "let mut data = Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    };",
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
                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 183,
                                                                                end: 187,
                                                                                as_str(): "data",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 183,
                                                                        end: 187,
                                                                        as_str(): "data",
                                                                    },
                                                                },
                                                                field_to_access: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 188,
                                                                        end: 193,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 183,
                                                            end: 193,
                                                            as_str(): "data.value",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
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
                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 196,
                                                                                    end: 210,
                                                                                    as_str(): "NumberOrString",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 196,
                                                                                end: 210,
                                                                                as_str(): "NumberOrString",
                                                                            },
                                                                        },
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 212,
                                                                                end: 218,
                                                                                as_str(): "String",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    is_absolute: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 196,
                                                                    end: 218,
                                                                    as_str(): "NumberOrString::String",
                                                                },
                                                            },
                                                            args: [
                                                                Expression {
                                                                    kind: Literal(
                                                                        String(
                                                                            Span {
                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 221,
                                                                                end: 225,
                                                                                as_str(): "sway",
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 220,
                                                                        end: 226,
                                                                        as_str(): "\"sway\"",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 196,
                                                        end: 227,
                                                        as_str(): "NumberOrString::String( \"sway\")",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056c4b800,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                            ),
                                            start: 183,
                                            end: 227,
                                            as_str(): "data.value = NumberOrString::String( \"sway\")",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 183,
                                    end: 227,
                                    as_str(): "data.value = NumberOrString::String( \"sway\")",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 240,
                                                    end: 241,
                                                    as_str(): "0",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056c4b800,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                            ),
                                            start: 233,
                                            end: 241,
                                            as_str(): "return 0",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 233,
                                    end: 241,
                                    as_str(): "return 0",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe056c4b800,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                            ),
                            start: 71,
                            end: 244,
                            as_str(): "{\n    let mut data = Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    };\n\n    data.value = NumberOrString::String( \"sway\");\n    return 0;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe056c4b800,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                        ),
                        start: 54,
                        end: 244,
                        as_str(): "fn main() -> u64 {\n    let mut data = Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    };\n\n    data.value = NumberOrString::String( \"sway\");\n    return 0;\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe056c4b800,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                        ),
                        start: 67,
                        end: 70,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe056c4b800,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
            ),
            start: 54,
            end: 244,
            as_str(): "fn main() -> u64 {\n    let mut data = Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    };\n\n    data.value = NumberOrString::String( \"sway\");\n    return 0;\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe056c4b800,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                            ),
                            start: 251,
                            end: 265,
                            as_str(): "NumberOrString",
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
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 272,
                                    end: 278,
                                    as_str(): "Number",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe056c4b800,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                ),
                                start: 280,
                                end: 283,
                                as_str(): "u64",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe056c4b800,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                ),
                                start: 272,
                                end: 283,
                                as_str(): "Number: u64",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 289,
                                    end: 295,
                                    as_str(): "String",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Str(
                                Length {
                                    val: 4,
                                    span: Span {
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 301,
                                        end: 302,
                                        as_str(): "4",
                                    },
                                },
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe056c4b800,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                ),
                                start: 297,
                                end: 303,
                                as_str(): "str[4]",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe056c4b800,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                ),
                                start: 289,
                                end: 303,
                                as_str(): "String: str[4]",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe056c4b800,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                        ),
                        start: 246,
                        end: 306,
                        as_str(): "enum NumberOrString {\n    Number: u64,\n    String: str[4],\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe056c4b800,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
            ),
            start: 246,
            end: 306,
            as_str(): "enum NumberOrString {\n    Number: u64,\n    String: str[4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe056c4b800,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                            ),
                            start: 315,
                            end: 319,
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
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 326,
                                    end: 331,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 333,
                                        end: 347,
                                        as_str(): "NumberOrString",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe056c4b800,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                ),
                                start: 326,
                                end: 347,
                                as_str(): "value: NumberOrString",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe056c4b800,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                ),
                                start: 333,
                                end: 347,
                                as_str(): "NumberOrString",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 353,
                                    end: 360,
                                    as_str(): "address",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                Eight,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe056c4b800,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                ),
                                start: 353,
                                end: 364,
                                as_str(): "address: u8",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe056c4b800,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                ),
                                start: 362,
                                end: 364,
                                as_str(): "u8",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe056c4b800,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                        ),
                        start: 308,
                        end: 367,
                        as_str(): "struct Data {\n    value: NumberOrString,\n    address: u8,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe056c4b800,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
            ),
            start: 308,
            end: 367,
            as_str(): "struct Data {\n    value: NumberOrString,\n    address: u8,\n}",
        },
    },
]
