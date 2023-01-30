TyStructDeclaration {
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
    attributes: {},
}
TyEnumDeclaration {
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
    type_parameters: [],
    attributes: {},
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
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
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
                                    fields: [],
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
                                return_type: TypeId(
                                    7254,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7254,
                            ),
                            type_ascription: TypeId(
                                7252,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                10,
                            ),
                        ),
                        return_type: TypeId(
                            21,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a0e6187a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
        ),
        start: 66,
        end: 141,
        as_str(): "fn main() -> u64 {\n    let unit_struct = StructWithNoFields {};\n    10u64\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

