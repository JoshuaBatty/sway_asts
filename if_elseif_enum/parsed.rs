[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 12,
                            end: 16,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 8,
            end: 20,
            as_str(): "use core::*;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 25,
                            end: 29,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 31,
                            end: 34,
                            as_str(): "ops",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 36,
                            end: 39,
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
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 21,
            end: 40,
            as_str(): "use core::ops::Ord;",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 49,
                            end: 52,
                            as_str(): "Rgb",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 62,
                                    as_str(): "red",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 59,
                                end: 67,
                                as_str(): "red: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 64,
                                end: 67,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 73,
                                    end: 78,
                                    as_str(): "green",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 73,
                                end: 83,
                                as_str(): "green: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 80,
                                end: 83,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 89,
                                    end: 93,
                                    as_str(): "blue",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 89,
                                end: 98,
                                as_str(): "blue: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 95,
                                end: 98,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 42,
                        end: 101,
                        as_str(): "struct Rgb {\n    red: u64,\n    green: u64,\n    blue: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 42,
            end: 101,
            as_str(): "struct Rgb {\n    red: u64,\n    green: u64,\n    blue: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 109,
                            end: 114,
                            as_str(): "Color",
                        },
                        is_raw_ident: false,
                    },
                    type_parameters: [],
                    attributes: {},
                    interface_surface: [
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 124,
                                    end: 127,
                                    as_str(): "rgb",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 132,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 128,
                                        end: 132,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 137,
                                        end: 140,
                                        as_str(): "Rgb",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 137,
                                end: 140,
                                as_str(): "Rgb",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 103,
                        end: 143,
                        as_str(): "trait Color {\n    fn rgb(self) -> Rgb;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 103,
            end: 143,
            as_str(): "trait Color {\n    fn rgb(self) -> Rgb;\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 150,
                            end: 162,
                            as_str(): "PrimaryColor",
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
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 169,
                                    end: 172,
                                    as_str(): "Red",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 174,
                                end: 176,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 169,
                                end: 176,
                                as_str(): "Red: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 182,
                                    end: 187,
                                    as_str(): "Green",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 189,
                                end: 191,
                                as_str(): "()",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 182,
                                end: 191,
                                as_str(): "Green: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 197,
                                    end: 201,
                                    as_str(): "Blue",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 203,
                                end: 205,
                                as_str(): "()",
                            },
                            tag: 2,
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 197,
                                end: 205,
                                as_str(): "Blue: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 145,
                        end: 208,
                        as_str(): "enum PrimaryColor {\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 145,
            end: 208,
            as_str(): "enum PrimaryColor {\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 215,
                                    end: 219,
                                    as_str(): "core",
                                },
                                is_raw_ident: false,
                            },
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 221,
                                    end: 224,
                                    as_str(): "ops",
                                },
                                is_raw_ident: false,
                            },
                        ],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 226,
                                end: 228,
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
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 233,
                                end: 245,
                                as_str(): "PrimaryColor",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 233,
                        end: 245,
                        as_str(): "PrimaryColor",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 255,
                                    end: 257,
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
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 299,
                                                                        end: 301,
                                                                        as_str(): "r1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: Some(
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 303,
                                                                                    end: 307,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 303,
                                                                            end: 307,
                                                                            as_str(): "self",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 309,
                                                                        end: 311,
                                                                        as_str(): "r2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: Some(
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 313,
                                                                                    end: 318,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 313,
                                                                            end: 318,
                                                                            as_str(): "other",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 320,
                                                                        end: 322,
                                                                        as_str(): "r3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: None,
                                                            },
                                                        ],
                                                        body: [
                                                            AsmOp {
                                                                op_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 338,
                                                                        end: 340,
                                                                        as_str(): "eq",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                op_args: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 341,
                                                                            end: 343,
                                                                            as_str(): "r3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 344,
                                                                            end: 346,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 347,
                                                                            end: 349,
                                                                            as_str(): "r2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 338,
                                                                    end: 349,
                                                                    as_str(): "eq r3 r1 r2",
                                                                },
                                                                immediate: None,
                                                            },
                                                        ],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "r3",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 363,
                                                                    end: 365,
                                                                    as_str(): "r3",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: Boolean,
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 381,
                                                            as_str(): "asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 295,
                                                    end: 381,
                                                    as_str(): "asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 295,
                                            end: 381,
                                            as_str(): "asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 285,
                                    end: 387,
                                    as_str(): "{\n        asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 258,
                                            end: 262,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 258,
                                        end: 262,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 264,
                                            end: 269,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 271,
                                        end: 275,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 252,
                                end: 387,
                                as_str(): "fn eq(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 280,
                                end: 284,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 210,
                        end: 389,
                        as_str(): "impl core::ops::Eq for PrimaryColor {\n    fn eq(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 210,
            end: 389,
            as_str(): "impl core::ops::Eq for PrimaryColor {\n    fn eq(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 396,
                                    end: 400,
                                    as_str(): "core",
                                },
                                is_raw_ident: false,
                            },
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 402,
                                    end: 405,
                                    as_str(): "ops",
                                },
                                is_raw_ident: false,
                            },
                        ],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 407,
                                end: 410,
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
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 415,
                                end: 427,
                                as_str(): "PrimaryColor",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 415,
                        end: 427,
                        as_str(): "PrimaryColor",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 437,
                                    end: 439,
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
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 481,
                                                                        end: 483,
                                                                        as_str(): "r1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: Some(
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 485,
                                                                                    end: 489,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 485,
                                                                            end: 489,
                                                                            as_str(): "self",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 491,
                                                                        end: 493,
                                                                        as_str(): "r2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: Some(
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 495,
                                                                                    end: 500,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 495,
                                                                            end: 500,
                                                                            as_str(): "other",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 502,
                                                                        end: 504,
                                                                        as_str(): "r3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: None,
                                                            },
                                                        ],
                                                        body: [
                                                            AsmOp {
                                                                op_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 520,
                                                                        end: 522,
                                                                        as_str(): "lt",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                op_args: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 523,
                                                                            end: 525,
                                                                            as_str(): "r3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 526,
                                                                            end: 528,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 529,
                                                                            end: 531,
                                                                            as_str(): "r2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 520,
                                                                    end: 531,
                                                                    as_str(): "lt r3 r1 r2",
                                                                },
                                                                immediate: None,
                                                            },
                                                        ],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "r3",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 545,
                                                                    end: 547,
                                                                    as_str(): "r3",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: Boolean,
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 477,
                                                            end: 563,
                                                            as_str(): "asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 477,
                                                    end: 563,
                                                    as_str(): "asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 477,
                                            end: 563,
                                            as_str(): "asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 467,
                                    end: 569,
                                    as_str(): "{\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 440,
                                            end: 444,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 440,
                                        end: 444,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 446,
                                            end: 451,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 453,
                                        end: 457,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 434,
                                end: 569,
                                as_str(): "fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 462,
                                end: 466,
                                as_str(): "bool",
                            },
                        },
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 577,
                                    end: 579,
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
                                                kind: Asm(
                                                    AsmExpression {
                                                        registers: [
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 621,
                                                                        end: 623,
                                                                        as_str(): "r1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: Some(
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 625,
                                                                                    end: 629,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 625,
                                                                            end: 629,
                                                                            as_str(): "self",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 631,
                                                                        end: 633,
                                                                        as_str(): "r2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: Some(
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 635,
                                                                                    end: 640,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 635,
                                                                            end: 640,
                                                                            as_str(): "other",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 642,
                                                                        end: 644,
                                                                        as_str(): "r3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                initializer: None,
                                                            },
                                                        ],
                                                        body: [
                                                            AsmOp {
                                                                op_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 660,
                                                                        end: 662,
                                                                        as_str(): "gt",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                op_args: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 663,
                                                                            end: 665,
                                                                            as_str(): "r3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 666,
                                                                            end: 668,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 669,
                                                                            end: 671,
                                                                            as_str(): "r2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 660,
                                                                    end: 671,
                                                                    as_str(): "gt r3 r1 r2",
                                                                },
                                                                immediate: None,
                                                            },
                                                        ],
                                                        returns: Some(
                                                            (
                                                                AsmRegister {
                                                                    name: "r3",
                                                                },
                                                                Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 685,
                                                                    end: 687,
                                                                    as_str(): "r3",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: Boolean,
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 617,
                                                            end: 703,
                                                            as_str(): "asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 617,
                                                    end: 703,
                                                    as_str(): "asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 617,
                                            end: 703,
                                            as_str(): "asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 607,
                                    end: 709,
                                    as_str(): "{\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 580,
                                            end: 584,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 580,
                                        end: 584,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 586,
                                            end: 591,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 593,
                                        end: 597,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 574,
                                end: 709,
                                as_str(): "fn gt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 602,
                                end: 606,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 391,
                        end: 711,
                        as_str(): "impl core::ops::Ord for PrimaryColor {\n    fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }\n    fn gt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 391,
            end: 711,
            as_str(): "impl core::ops::Ord for PrimaryColor {\n    fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }\n    fn gt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }\n}",
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
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 718,
                                end: 723,
                                as_str(): "Color",
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
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 728,
                                end: 740,
                                as_str(): "PrimaryColor",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 728,
                        end: 740,
                        as_str(): "PrimaryColor",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 830,
                                    end: 833,
                                    as_str(): "rgb",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: If(
                                                    IfExpression {
                                                        condition: Expression {
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
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 865,
                                                                                            end: 867,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 865,
                                                                                            end: 867,
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
                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 865,
                                                                                        end: 867,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 865,
                                                                            end: 867,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 860,
                                                                                        end: 864,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 860,
                                                                                end: 864,
                                                                                as_str(): "self",
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
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 868,
                                                                                                        end: 880,
                                                                                                        as_str(): "PrimaryColor",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 882,
                                                                                                    end: 885,
                                                                                                    as_str(): "Red",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 882,
                                                                                            end: 885,
                                                                                            as_str(): "Red",
                                                                                        },
                                                                                    },
                                                                                    args: None,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 868,
                                                                                end: 885,
                                                                                as_str(): "PrimaryColor::Red",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 860,
                                                                end: 885,
                                                                as_str(): "self == PrimaryColor::Red",
                                                            },
                                                        },
                                                        then: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
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
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 900,
                                                                                                            end: 903,
                                                                                                            as_str(): "Rgb",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 900,
                                                                                                    end: 903,
                                                                                                    as_str(): "Rgb",
                                                                                                },
                                                                                            },
                                                                                            fields: [
                                                                                                StructExpressionField {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 922,
                                                                                                            end: 925,
                                                                                                            as_str(): "red",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    value: Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                255,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 927,
                                                                                                            end: 930,
                                                                                                            as_str(): "255",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 922,
                                                                                                        end: 930,
                                                                                                        as_str(): "red: 255",
                                                                                                    },
                                                                                                },
                                                                                                StructExpressionField {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 948,
                                                                                                            end: 952,
                                                                                                            as_str(): "blue",
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
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 954,
                                                                                                            end: 955,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 948,
                                                                                                        end: 955,
                                                                                                        as_str(): "blue: 0",
                                                                                                    },
                                                                                                },
                                                                                                StructExpressionField {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 973,
                                                                                                            end: 978,
                                                                                                            as_str(): "green",
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
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 980,
                                                                                                            end: 981,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 973,
                                                                                                        end: 981,
                                                                                                        as_str(): "green: 0",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 900,
                                                                                        end: 996,
                                                                                        as_str(): "Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 900,
                                                                                end: 996,
                                                                                as_str(): "Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 886,
                                                                        end: 1006,
                                                                        as_str(): "{\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 886,
                                                                end: 1006,
                                                                as_str(): "{\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        }",
                                                            },
                                                        },
                                                        else: Some(
                                                            Expression {
                                                                kind: If(
                                                                    IfExpression {
                                                                        condition: Expression {
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
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1020,
                                                                                                            end: 1022,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ops",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1020,
                                                                                                            end: 1022,
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
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1020,
                                                                                                        end: 1022,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: true,
                                                                                            },
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 1020,
                                                                                            end: 1022,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                    },
                                                                                    contract_call_params: [],
                                                                                    arguments: [
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1015,
                                                                                                        end: 1019,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 1015,
                                                                                                end: 1019,
                                                                                                as_str(): "self",
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
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1023,
                                                                                                                        end: 1035,
                                                                                                                        as_str(): "PrimaryColor",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1037,
                                                                                                                    end: 1042,
                                                                                                                    as_str(): "Green",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1037,
                                                                                                            end: 1042,
                                                                                                            as_str(): "Green",
                                                                                                        },
                                                                                                    },
                                                                                                    args: None,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 1023,
                                                                                                end: 1042,
                                                                                                as_str(): "PrimaryColor::Green",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1015,
                                                                                end: 1042,
                                                                                as_str(): "self == PrimaryColor::Green",
                                                                            },
                                                                        },
                                                                        then: Expression {
                                                                            kind: CodeBlock(
                                                                                CodeBlock {
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
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1057,
                                                                                                                            end: 1060,
                                                                                                                            as_str(): "Rgb",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    is_absolute: false,
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1057,
                                                                                                                    end: 1060,
                                                                                                                    as_str(): "Rgb",
                                                                                                                },
                                                                                                            },
                                                                                                            fields: [
                                                                                                                StructExpressionField {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1079,
                                                                                                                            end: 1082,
                                                                                                                            as_str(): "red",
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
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1084,
                                                                                                                            end: 1085,
                                                                                                                            as_str(): "0",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1079,
                                                                                                                        end: 1085,
                                                                                                                        as_str(): "red: 0",
                                                                                                                    },
                                                                                                                },
                                                                                                                StructExpressionField {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1103,
                                                                                                                            end: 1107,
                                                                                                                            as_str(): "blue",
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
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1109,
                                                                                                                            end: 1110,
                                                                                                                            as_str(): "0",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1103,
                                                                                                                        end: 1110,
                                                                                                                        as_str(): "blue: 0",
                                                                                                                    },
                                                                                                                },
                                                                                                                StructExpressionField {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1128,
                                                                                                                            end: 1133,
                                                                                                                            as_str(): "green",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    value: Expression {
                                                                                                                        kind: Literal(
                                                                                                                            Numeric(
                                                                                                                                255,
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1135,
                                                                                                                            end: 1138,
                                                                                                                            as_str(): "255",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1128,
                                                                                                                        end: 1138,
                                                                                                                        as_str(): "green: 255",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1057,
                                                                                                        end: 1153,
                                                                                                        as_str(): "Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 1057,
                                                                                                end: 1153,
                                                                                                as_str(): "Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                    whole_block_span: Span {
                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 1043,
                                                                                        end: 1163,
                                                                                        as_str(): "{\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1043,
                                                                                end: 1163,
                                                                                as_str(): "{\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        }",
                                                                            },
                                                                        },
                                                                        else: Some(
                                                                            Expression {
                                                                                kind: If(
                                                                                    IfExpression {
                                                                                        condition: Expression {
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
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1177,
                                                                                                                            end: 1179,
                                                                                                                            as_str(): "==",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1177,
                                                                                                                            end: 1179,
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
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1177,
                                                                                                                        end: 1179,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                is_absolute: true,
                                                                                                            },
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1177,
                                                                                                            end: 1179,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                    },
                                                                                                    contract_call_params: [],
                                                                                                    arguments: [
                                                                                                        Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1172,
                                                                                                                        end: 1176,
                                                                                                                        as_str(): "self",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1172,
                                                                                                                end: 1176,
                                                                                                                as_str(): "self",
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
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1180,
                                                                                                                                        end: 1192,
                                                                                                                                        as_str(): "PrimaryColor",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ],
                                                                                                                            suffix: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1194,
                                                                                                                                    end: 1198,
                                                                                                                                    as_str(): "Blue",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            is_absolute: false,
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1194,
                                                                                                                            end: 1198,
                                                                                                                            as_str(): "Blue",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    args: None,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1180,
                                                                                                                end: 1198,
                                                                                                                as_str(): "PrimaryColor::Blue",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 1172,
                                                                                                end: 1198,
                                                                                                as_str(): "self == PrimaryColor::Blue",
                                                                                            },
                                                                                        },
                                                                                        then: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
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
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1213,
                                                                                                                                            end: 1216,
                                                                                                                                            as_str(): "Rgb",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    is_absolute: false,
                                                                                                                                },
                                                                                                                                type_arguments: [],
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1213,
                                                                                                                                    end: 1216,
                                                                                                                                    as_str(): "Rgb",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            fields: [
                                                                                                                                StructExpressionField {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1235,
                                                                                                                                            end: 1238,
                                                                                                                                            as_str(): "red",
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
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1240,
                                                                                                                                            end: 1241,
                                                                                                                                            as_str(): "0",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1235,
                                                                                                                                        end: 1241,
                                                                                                                                        as_str(): "red: 0",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                StructExpressionField {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1259,
                                                                                                                                            end: 1263,
                                                                                                                                            as_str(): "blue",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    value: Expression {
                                                                                                                                        kind: Literal(
                                                                                                                                            Numeric(
                                                                                                                                                255,
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1265,
                                                                                                                                            end: 1268,
                                                                                                                                            as_str(): "255",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1259,
                                                                                                                                        end: 1268,
                                                                                                                                        as_str(): "blue: 255",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                StructExpressionField {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1286,
                                                                                                                                            end: 1291,
                                                                                                                                            as_str(): "green",
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
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1293,
                                                                                                                                            end: 1294,
                                                                                                                                            as_str(): "0",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1286,
                                                                                                                                        end: 1294,
                                                                                                                                        as_str(): "green: 0",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ],
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1213,
                                                                                                                        end: 1309,
                                                                                                                        as_str(): "Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1213,
                                                                                                                end: 1309,
                                                                                                                as_str(): "Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1199,
                                                                                                        end: 1319,
                                                                                                        as_str(): "{\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 1199,
                                                                                                end: 1319,
                                                                                                as_str(): "{\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }",
                                                                                            },
                                                                                        },
                                                                                        else: Some(
                                                                                            Expression {
                                                                                                kind: CodeBlock(
                                                                                                    CodeBlock {
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
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1416,
                                                                                                                                                end: 1419,
                                                                                                                                                as_str(): "Rgb",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        is_absolute: false,
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1416,
                                                                                                                                        end: 1419,
                                                                                                                                        as_str(): "Rgb",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                fields: [
                                                                                                                                    StructExpressionField {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1438,
                                                                                                                                                end: 1441,
                                                                                                                                                as_str(): "red",
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
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1443,
                                                                                                                                                end: 1444,
                                                                                                                                                as_str(): "0",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1438,
                                                                                                                                            end: 1444,
                                                                                                                                            as_str(): "red: 0",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    StructExpressionField {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1462,
                                                                                                                                                end: 1467,
                                                                                                                                                as_str(): "green",
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
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1469,
                                                                                                                                                end: 1470,
                                                                                                                                                as_str(): "0",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1462,
                                                                                                                                            end: 1470,
                                                                                                                                            as_str(): "green: 0",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    StructExpressionField {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1488,
                                                                                                                                                end: 1492,
                                                                                                                                                as_str(): "blue",
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
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1494,
                                                                                                                                                end: 1495,
                                                                                                                                                as_str(): "0",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1488,
                                                                                                                                            end: 1495,
                                                                                                                                            as_str(): "blue: 0",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1416,
                                                                                                                            end: 1510,
                                                                                                                            as_str(): "Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1416,
                                                                                                                    end: 1510,
                                                                                                                    as_str(): "Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }",
                                                                                                                },
                                                                                                            },
                                                                                                        ],
                                                                                                        whole_block_span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1402,
                                                                                                            end: 1520,
                                                                                                            as_str(): "{\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1402,
                                                                                                    end: 1520,
                                                                                                    as_str(): "{\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 1169,
                                                                                    end: 1520,
                                                                                    as_str(): "if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1012,
                                                                    end: 1520,
                                                                    as_str(): "if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 857,
                                                    end: 1520,
                                                    as_str(): "if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 857,
                                            end: 1520,
                                            as_str(): "if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 847,
                                    end: 1526,
                                    as_str(): "{\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 834,
                                            end: 838,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 834,
                                        end: 838,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 827,
                                end: 1526,
                                as_str(): "fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }",
                            },
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 843,
                                        end: 846,
                                        as_str(): "Rgb",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fb117fe0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                ),
                                start: 843,
                                end: 846,
                                as_str(): "Rgb",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 713,
                        end: 1528,
                        as_str(): "impl Color for PrimaryColor {\n    // TODO: when we support match statements, change this to a match statement\n    fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 713,
            end: 1528,
            as_str(): "impl Color for PrimaryColor {\n    // TODO: when we support match statements, change this to a match statement\n    fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }\n}",
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
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 1533,
                            end: 1537,
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1557,
                                                    end: 1568,
                                                    as_str(): "first_color",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 1570,
                                                        end: 1582,
                                                        as_str(): "PrimaryColor",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1570,
                                                    end: 1582,
                                                    as_str(): "PrimaryColor",
                                                },
                                            ),
                                            body: Expression {
                                                kind: DelineatedPath(
                                                    DelineatedPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1585,
                                                                            end: 1597,
                                                                            as_str(): "PrimaryColor",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 1599,
                                                                        end: 1604,
                                                                        as_str(): "Green",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1599,
                                                                end: 1604,
                                                                as_str(): "Green",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1585,
                                                    end: 1604,
                                                    as_str(): "PrimaryColor::Green",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1553,
                                    end: 1605,
                                    as_str(): "let first_color: PrimaryColor = PrimaryColor::Green;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1614,
                                                    end: 1618,
                                                    as_str(): "test",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1633,
                                                                                end: 1635,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1633,
                                                                                end: 1635,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1633,
                                                                            end: 1635,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1633,
                                                                end: 1635,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1621,
                                                                            end: 1632,
                                                                            as_str(): "first_color",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1621,
                                                                    end: 1632,
                                                                    as_str(): "first_color",
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
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 1636,
                                                                                            end: 1648,
                                                                                            as_str(): "PrimaryColor",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 1650,
                                                                                        end: 1655,
                                                                                        as_str(): "Green",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1650,
                                                                                end: 1655,
                                                                                as_str(): "Green",
                                                                            },
                                                                        },
                                                                        args: None,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1636,
                                                                    end: 1655,
                                                                    as_str(): "PrimaryColor::Green",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1621,
                                                    end: 1655,
                                                    as_str(): "first_color == PrimaryColor::Green",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1610,
                                    end: 1656,
                                    as_str(): "let test = first_color == PrimaryColor::Green;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1745,
                                                    end: 1748,
                                                    as_str(): "rgb",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 1750,
                                                        end: 1753,
                                                        as_str(): "Rgb",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1750,
                                                    end: 1753,
                                                    as_str(): "Rgb",
                                                },
                                            ),
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 1768,
                                                                        end: 1771,
                                                                        as_str(): "rgb",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1768,
                                                                end: 1771,
                                                                as_str(): "rgb",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1756,
                                                                            end: 1767,
                                                                            as_str(): "first_color",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1756,
                                                                    end: 1767,
                                                                    as_str(): "first_color",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1756,
                                                    end: 1773,
                                                    as_str(): "first_color.rgb()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1741,
                                    end: 1774,
                                    as_str(): "let rgb: Rgb = first_color.rgb();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1854,
                                                    end: 1866,
                                                    as_str(): "second_color",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: DelineatedPath(
                                                    DelineatedPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1869,
                                                                            end: 1881,
                                                                            as_str(): "PrimaryColor",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 1883,
                                                                        end: 1887,
                                                                        as_str(): "Blue",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1883,
                                                                end: 1887,
                                                                as_str(): "Blue",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1869,
                                                    end: 1887,
                                                    as_str(): "PrimaryColor::Blue",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1850,
                                    end: 1888,
                                    as_str(): "let second_color = PrimaryColor::Blue;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1897,
                                                    end: 1907,
                                                    as_str(): "second_rgb",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 1923,
                                                                        end: 1926,
                                                                        as_str(): "rgb",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1923,
                                                                end: 1926,
                                                                as_str(): "rgb",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1910,
                                                                            end: 1922,
                                                                            as_str(): "second_color",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1910,
                                                                    end: 1922,
                                                                    as_str(): "second_color",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1910,
                                                    end: 1928,
                                                    as_str(): "second_color.rgb()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1893,
                                    end: 1929,
                                    as_str(): "let second_rgb = second_color.rgb();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1938,
                                                    end: 1950,
                                                    as_str(): "second_color",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: DelineatedPath(
                                                    DelineatedPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1953,
                                                                            end: 1965,
                                                                            as_str(): "PrimaryColor",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 1967,
                                                                        end: 1971,
                                                                        as_str(): "Blue",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1967,
                                                                end: 1971,
                                                                as_str(): "Blue",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1953,
                                                    end: 1971,
                                                    as_str(): "PrimaryColor::Blue",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1934,
                                    end: 1972,
                                    as_str(): "let second_color = PrimaryColor::Blue;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1981,
                                                    end: 1991,
                                                    as_str(): "second_rgb",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 2007,
                                                                        end: 2010,
                                                                        as_str(): "rgb",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2007,
                                                                end: 2010,
                                                                as_str(): "rgb",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1994,
                                                                            end: 2006,
                                                                            as_str(): "second_color",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1994,
                                                                    end: 2006,
                                                                    as_str(): "second_color",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1994,
                                                    end: 2012,
                                                    as_str(): "second_color.rgb()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1977,
                                    end: 2013,
                                    as_str(): "let second_rgb = second_color.rgb();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 2022,
                                                    end: 2034,
                                                    as_str(): "second_color",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: DelineatedPath(
                                                    DelineatedPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 2037,
                                                                            end: 2049,
                                                                            as_str(): "PrimaryColor",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 2051,
                                                                        end: 2055,
                                                                        as_str(): "Blue",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2051,
                                                                end: 2055,
                                                                as_str(): "Blue",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 2037,
                                                    end: 2055,
                                                    as_str(): "PrimaryColor::Blue",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 2018,
                                    end: 2056,
                                    as_str(): "let second_color = PrimaryColor::Blue;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 2065,
                                                    end: 2075,
                                                    as_str(): "second_rgb",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 2091,
                                                                        end: 2094,
                                                                        as_str(): "rgb",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2091,
                                                                end: 2094,
                                                                as_str(): "rgb",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 2078,
                                                                            end: 2090,
                                                                            as_str(): "second_color",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 2078,
                                                                    end: 2090,
                                                                    as_str(): "second_color",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 2078,
                                                    end: 2096,
                                                    as_str(): "second_color.rgb()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 2061,
                                    end: 2097,
                                    as_str(): "let second_rgb = second_color.rgb();",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            U32(
                                                10,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 2102,
                                            end: 2107,
                                            as_str(): "10u32",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 2102,
                                    end: 2107,
                                    as_str(): "10u32",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 1547,
                            end: 2109,
                            as_str(): "{\n    let first_color: PrimaryColor = PrimaryColor::Green;\n    let test = first_color == PrimaryColor::Green;\n    // Specifically, when we call methods in the below way, `self` is undefined\n    let rgb: Rgb = first_color.rgb();\n    // now, going to test the register pool by using over 48 registers\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    10u32\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 1530,
                        end: 2109,
                        as_str(): "fn main() -> u32 {\n    let first_color: PrimaryColor = PrimaryColor::Green;\n    let test = first_color == PrimaryColor::Green;\n    // Specifically, when we call methods in the below way, `self` is undefined\n    let rgb: Rgb = first_color.rgb();\n    // now, going to test the register pool by using over 48 registers\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    10u32\n}",
                    },
                    return_type: UnsignedInteger(
                        ThirtyTwo,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                        ),
                        start: 1543,
                        end: 1546,
                        as_str(): "u32",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 1530,
            end: 2109,
            as_str(): "fn main() -> u32 {\n    let first_color: PrimaryColor = PrimaryColor::Green;\n    let test = first_color == PrimaryColor::Green;\n    // Specifically, when we call methods in the below way, `self` is undefined\n    let rgb: Rgb = first_color.rgb();\n    // now, going to test the register pool by using over 48 registers\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    10u32\n}",
        },
    },
]
