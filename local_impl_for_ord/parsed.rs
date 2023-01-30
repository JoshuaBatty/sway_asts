[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 13,
                            end: 17,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 19,
                            end: 22,
                            as_str(): "ops",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 25,
                            end: 27,
                            as_str(): "Eq",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0bc9cf310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
            ),
            start: 9,
            end: 34,
            as_str(): "use core::ops::{Eq, Ord};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 13,
                            end: 17,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 19,
                            end: 22,
                            as_str(): "ops",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 29,
                            end: 32,
                            as_str(): "Ord",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0bc9cf310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
            ),
            start: 9,
            end: 34,
            as_str(): "use core::ops::{Eq, Ord};",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 41,
                            end: 42,
                            as_str(): "X",
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
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 49,
                                    end: 50,
                                    as_str(): "Y",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 52,
                                end: 54,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 49,
                                end: 54,
                                as_str(): "Y: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0bc9cf310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                        ),
                        start: 36,
                        end: 57,
                        as_str(): "enum X {\n    Y: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0bc9cf310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
            ),
            start: 36,
            end: 57,
            as_str(): "enum X {\n    Y: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 64,
                                end: 66,
                                as_str(): "Eq",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 71,
                                end: 72,
                                as_str(): "X",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0bc9cf310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                        ),
                        start: 71,
                        end: 72,
                        as_str(): "X",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 82,
                                    end: 84,
                                    as_str(): "eq",
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
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bc9cf310,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 126,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 122,
                                            end: 126,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 112,
                                    end: 132,
                                    as_str(): "{\n        true\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 85,
                                            end: 89,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 85,
                                        end: 89,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 91,
                                            end: 96,
                                            as_str(): "other",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 98,
                                        end: 102,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 79,
                                end: 132,
                                as_str(): "fn eq(self, other: Self) -> bool {\n        true\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 107,
                                end: 111,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0bc9cf310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                        ),
                        start: 59,
                        end: 134,
                        as_str(): "impl Eq for X {\n    fn eq(self, other: Self) -> bool {\n        true\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0bc9cf310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
            ),
            start: 59,
            end: 134,
            as_str(): "impl Eq for X {\n    fn eq(self, other: Self) -> bool {\n        true\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 141,
                                end: 144,
                                as_str(): "Ord",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 149,
                                end: 150,
                                as_str(): "X",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0bc9cf310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                        ),
                        start: 149,
                        end: 150,
                        as_str(): "X",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 160,
                                    end: 162,
                                    as_str(): "lt",
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
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bc9cf310,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                    ),
                                                    start: 200,
                                                    end: 205,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 200,
                                            end: 205,
                                            as_str(): "false",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 190,
                                    end: 211,
                                    as_str(): "{\n        false\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 163,
                                            end: 167,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 163,
                                        end: 167,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 169,
                                            end: 174,
                                            as_str(): "other",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 176,
                                        end: 180,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 157,
                                end: 211,
                                as_str(): "fn lt(self, other: Self) -> bool {\n        false\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 185,
                                end: 189,
                                as_str(): "bool",
                            },
                        },
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 219,
                                    end: 221,
                                    as_str(): "gt",
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
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bc9cf310,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                    ),
                                                    start: 259,
                                                    end: 264,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 259,
                                            end: 264,
                                            as_str(): "false",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 249,
                                    end: 270,
                                    as_str(): "{\n        false\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 222,
                                            end: 226,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 222,
                                        end: 226,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 228,
                                            end: 233,
                                            as_str(): "other",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 235,
                                        end: 239,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 216,
                                end: 270,
                                as_str(): "fn gt(self, other: Self) -> bool {\n        false\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0bc9cf310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                ),
                                start: 244,
                                end: 248,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0bc9cf310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                        ),
                        start: 136,
                        end: 272,
                        as_str(): "impl Ord for X {\n    fn lt(self, other: Self) -> bool {\n        false\n    }\n    fn gt(self, other: Self) -> bool {\n        false\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0bc9cf310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
            ),
            start: 136,
            end: 272,
            as_str(): "impl Ord for X {\n    fn lt(self, other: Self) -> bool {\n        false\n    }\n    fn gt(self, other: Self) -> bool {\n        false\n    }\n}",
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
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 277,
                            end: 281,
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
                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                        ),
                                                                        start: 303,
                                                                        end: 305,
                                                                        as_str(): "==",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "ops",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                        ),
                                                                        start: 303,
                                                                        end: 305,
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
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 303,
                                                                    end: 305,
                                                                    as_str(): "==",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_absolute: true,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bc9cf310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                        ),
                                                        start: 303,
                                                        end: 305,
                                                        as_str(): "==",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: DelineatedPath(
                                                            DelineatedPathExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                    ),
                                                                                    start: 298,
                                                                                    end: 299,
                                                                                    as_str(): "X",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bc9cf310,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                ),
                                                                                start: 301,
                                                                                end: 302,
                                                                                as_str(): "Y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                        ),
                                                                        start: 301,
                                                                        end: 302,
                                                                        as_str(): "Y",
                                                                    },
                                                                },
                                                                args: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 302,
                                                            as_str(): "X::Y",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: DelineatedPath(
                                                            DelineatedPathExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                    ),
                                                                                    start: 306,
                                                                                    end: 307,
                                                                                    as_str(): "X",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bc9cf310,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                ),
                                                                                start: 309,
                                                                                end: 310,
                                                                                as_str(): "Y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                        ),
                                                                        start: 309,
                                                                        end: 310,
                                                                        as_str(): "Y",
                                                                    },
                                                                },
                                                                args: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 306,
                                                            end: 310,
                                                            as_str(): "X::Y",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 298,
                                            end: 310,
                                            as_str(): "X::Y == X::Y",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 298,
                                    end: 310,
                                    as_str(): "X::Y == X::Y",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 292,
                            end: 312,
                            as_str(): "{\n    X::Y == X::Y\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0bc9cf310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                        ),
                        start: 274,
                        end: 312,
                        as_str(): "fn main() -> bool {\n    X::Y == X::Y\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0bc9cf310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                        ),
                        start: 287,
                        end: 291,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0bc9cf310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
            ),
            start: 274,
            end: 312,
            as_str(): "fn main() -> bool {\n    X::Y == X::Y\n}",
        },
    },
]
