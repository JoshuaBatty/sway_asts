[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe062436520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                            ),
                            start: 16,
                            end: 17,
                            as_str(): "A",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe062436520,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                    ),
                                    start: 24,
                                    end: 28,
                                    as_str(): "addr",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe062436520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                        ),
                                        start: 30,
                                        end: 37,
                                        as_str(): "Address",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe062436520,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                ),
                                start: 24,
                                end: 37,
                                as_str(): "addr: Address",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe062436520,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                ),
                                start: 30,
                                end: 37,
                                as_str(): "Address",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe062436520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                        ),
                        start: 9,
                        end: 40,
                        as_str(): "struct A {\n    addr: Address,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe062436520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
            ),
            start: 9,
            end: 40,
            as_str(): "struct A {\n    addr: Address,\n}",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe062436520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                            ),
                            start: 78,
                            end: 81,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe062436520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                            ),
                            start: 83,
                            end: 90,
                            as_str(): "address",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe062436520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                            ),
                            start: 92,
                            end: 99,
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
            src (ptr): 0x00007fe062436520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
            ),
            start: 74,
            end: 100,
            as_str(): "use std::address::Address;",
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
                            src (ptr): 0x00007fe062436520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                            ),
                            start: 105,
                            end: 109,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe062436520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                            ),
                            start: 112,
                            end: 115,
                            as_str(): "{\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe062436520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                        ),
                        start: 102,
                        end: 115,
                        as_str(): "fn main() {\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe062436520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                        ),
                        start: 102,
                        end: 111,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe062436520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
            ),
            start: 102,
            end: 115,
            as_str(): "fn main() {\n}",
        },
    },
]
