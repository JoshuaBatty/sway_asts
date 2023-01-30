[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fb1359d0020,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                    ),
                    start: 9,
                    end: 20,
                    as_str(): "dep consts;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fb1359d0020,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                    ),
                    start: 13,
                    end: 19,
                    as_str(): "consts",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fb1359d0020,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
            ),
            start: 9,
            end: 20,
            as_str(): "dep consts;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1359d0020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                            ),
                            start: 26,
                            end: 32,
                            as_str(): "consts",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1359d0020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                            ),
                            start: 34,
                            end: 39,
                            as_str(): "adder",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb1359d0020,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
            ),
            start: 22,
            end: 40,
            as_str(): "use consts::adder;",
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
                            src (ptr): 0x00007fb1359d0020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                            ),
                            start: 45,
                            end: 49,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1359d0020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 70,
                                                                as_str(): "adder",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1359d0020,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                                        ),
                                                        start: 65,
                                                        end: 70,
                                                        as_str(): "adder",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1359d0020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                            ),
                                            start: 65,
                                            end: 72,
                                            as_str(): "adder()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1359d0020,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 72,
                                    as_str(): "adder()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1359d0020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                            ),
                            start: 59,
                            end: 74,
                            as_str(): "{\n    adder()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb1359d0020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                        ),
                        start: 42,
                        end: 74,
                        as_str(): "fn main() -> u64 {\n    adder()\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1359d0020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                        ),
                        start: 55,
                        end: 58,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1359d0020,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
            ),
            start: 42,
            end: 74,
            as_str(): "fn main() -> u64 {\n    adder()\n}",
        },
    },
]
