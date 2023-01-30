[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a0e6187a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                            ),
                            start: 16,
                            end: 34,
                            as_str(): "StructWithNoFields",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007f8a0e6187a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                        ),
                        start: 9,
                        end: 37,
                        as_str(): "struct StructWithNoFields {}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a0e6187a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
            ),
            start: 9,
            end: 37,
            as_str(): "struct StructWithNoFields {}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a0e6187a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                            ),
                            start: 43,
                            end: 61,
                            as_str(): "EnumWithNoVariants",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [],
                    span: Span {
                        src (ptr): 0x00007f8a0e6187a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                        ),
                        start: 38,
                        end: 64,
                        as_str(): "enum EnumWithNoVariants {}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a0e6187a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
            ),
            start: 38,
            end: 64,
            as_str(): "enum EnumWithNoVariants {}",
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
                            src (ptr): 0x00007f8a0e6187a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                            ),
                            start: 69,
                            end: 73,
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
                                                    src (ptr): 0x00007f8a0e6187a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                    ),
                                                    start: 93,
                                                    end: 104,
                                                    as_str(): "unit_struct",
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
                                                                        src (ptr): 0x00007f8a0e6187a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                                        ),
                                                                        start: 107,
                                                                        end: 125,
                                                                        as_str(): "StructWithNoFields",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6187a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                                ),
                                                                start: 107,
                                                                end: 125,
                                                                as_str(): "StructWithNoFields",
                                                            },
                                                        },
                                                        fields: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6187a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                    ),
                                                    start: 107,
                                                    end: 128,
                                                    as_str(): "StructWithNoFields {}",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6187a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                    ),
                                    start: 89,
                                    end: 129,
                                    as_str(): "let unit_struct = StructWithNoFields {};",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            U64(
                                                10,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6187a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                            ),
                                            start: 134,
                                            end: 139,
                                            as_str(): "10u64",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6187a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                    ),
                                    start: 134,
                                    end: 139,
                                    as_str(): "10u64",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a0e6187a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                            ),
                            start: 83,
                            end: 141,
                            as_str(): "{\n    let unit_struct = StructWithNoFields {};\n    10u64\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a0e6187a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                        ),
                        start: 66,
                        end: 141,
                        as_str(): "fn main() -> u64 {\n    let unit_struct = StructWithNoFields {};\n    10u64\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a0e6187a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                        ),
                        start: 79,
                        end: 82,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a0e6187a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
            ),
            start: 66,
            end: 141,
            as_str(): "fn main() -> u64 {\n    let unit_struct = StructWithNoFields {};\n    10u64\n}",
        },
    },
]
