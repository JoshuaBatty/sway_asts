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
                            src (ptr): 0x00007fb0fa919aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                            ),
                            start: 11,
                            end: 15,
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
                                                    src (ptr): 0x00007fb0fa919aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                    ),
                                                    start: 35,
                                                    end: 36,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        255,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0fa919aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                    ),
                                                    start: 39,
                                                    end: 42,
                                                    as_str(): "255",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0fa919aa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                    ),
                                    start: 31,
                                    end: 43,
                                    as_str(): "let a = 255;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    EnumDeclaration(
                                        EnumDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0fa919aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                    ),
                                                    start: 54,
                                                    end: 55,
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
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 66,
                                                            end: 67,
                                                            as_str(): "Y",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    attributes: {},
                                                    type_info: Boolean,
                                                    type_span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 69,
                                                        end: 73,
                                                        as_str(): "bool",
                                                    },
                                                    tag: 0,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 66,
                                                        end: 73,
                                                        as_str(): "Y: bool",
                                                    },
                                                },
                                            ],
                                            span: Span {
                                                src (ptr): 0x00007fb0fa919aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                ),
                                                start: 49,
                                                end: 80,
                                                as_str(): "enum X {\n        Y: bool,\n    }",
                                            },
                                            visibility: Private,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0fa919aa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                    ),
                                    start: 49,
                                    end: 80,
                                    as_str(): "enum X {\n        Y: bool,\n    }",
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
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 91,
                                                            end: 95,
                                                            as_str(): "core",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 97,
                                                            end: 100,
                                                            as_str(): "ops",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 102,
                                                        end: 104,
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
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 109,
                                                        end: 110,
                                                        as_str(): "X",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_implementing_for_span: Span {
                                                src (ptr): 0x00007fb0fa919aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                ),
                                                start: 109,
                                                end: 110,
                                                as_str(): "X",
                                            },
                                            functions: [
                                                FunctionDeclaration {
                                                    purity: Pure,
                                                    attributes: {},
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 124,
                                                            end: 126,
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
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 172,
                                                                                                end: 174,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 176,
                                                                                                            end: 180,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 176,
                                                                                                    end: 180,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    AsmRegisterDeclaration {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 182,
                                                                                                end: 184,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 186,
                                                                                                            end: 191,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 186,
                                                                                                    end: 191,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    AsmRegisterDeclaration {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 193,
                                                                                                end: 195,
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
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 215,
                                                                                                end: 217,
                                                                                                as_str(): "eq",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        op_args: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 218,
                                                                                                    end: 220,
                                                                                                    as_str(): "r3",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 221,
                                                                                                    end: 223,
                                                                                                    as_str(): "r2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 224,
                                                                                                    end: 226,
                                                                                                    as_str(): "r1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 215,
                                                                                            end: 226,
                                                                                            as_str(): "eq r3 r2 r1",
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
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 244,
                                                                                            end: 246,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                return_type: Boolean,
                                                                                whole_block_span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 168,
                                                                                    end: 266,
                                                                                    as_str(): "asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                            ),
                                                                            start: 168,
                                                                            end: 266,
                                                                            as_str(): "asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 168,
                                                                    end: 266,
                                                                    as_str(): "asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 154,
                                                            end: 276,
                                                            as_str(): "{\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }",
                                                        },
                                                    },
                                                    parameters: [
                                                        FunctionParameter {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 127,
                                                                    end: 131,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_reference: false,
                                                            is_mutable: false,
                                                            mutability_span: Span {
                                                                src (ptr): 0x00007fb14c011bb0,
                                                                path: None,
                                                                start: 0,
                                                                end: 0,
                                                                as_str(): "",
                                                            },
                                                            type_info: SelfType,
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 127,
                                                                end: 131,
                                                                as_str(): "self",
                                                            },
                                                        },
                                                        FunctionParameter {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 133,
                                                                    end: 138,
                                                                    as_str(): "other",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_reference: false,
                                                            is_mutable: false,
                                                            mutability_span: Span {
                                                                src (ptr): 0x00007fb14c011bb0,
                                                                path: None,
                                                                start: 0,
                                                                end: 0,
                                                                as_str(): "",
                                                            },
                                                            type_info: SelfType,
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 140,
                                                                end: 144,
                                                                as_str(): "Self",
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 121,
                                                        end: 276,
                                                        as_str(): "fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }",
                                                    },
                                                    return_type: Boolean,
                                                    type_parameters: [],
                                                    return_type_span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 149,
                                                        end: 153,
                                                        as_str(): "bool",
                                                    },
                                                },
                                            ],
                                            block_span: Span {
                                                src (ptr): 0x00007fb0fa919aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                ),
                                                start: 86,
                                                end: 282,
                                                as_str(): "impl core::ops::Eq for X {\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0fa919aa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                    ),
                                    start: 86,
                                    end: 282,
                                    as_str(): "impl core::ops::Eq for X {\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }",
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
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 293,
                                                            end: 297,
                                                            as_str(): "core",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 299,
                                                            end: 302,
                                                            as_str(): "ops",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 304,
                                                        end: 307,
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
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 312,
                                                        end: 313,
                                                        as_str(): "X",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_implementing_for_span: Span {
                                                src (ptr): 0x00007fb0fa919aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                ),
                                                start: 312,
                                                end: 313,
                                                as_str(): "X",
                                            },
                                            functions: [
                                                FunctionDeclaration {
                                                    purity: Pure,
                                                    attributes: {},
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 327,
                                                            end: 329,
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
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 375,
                                                                                                end: 377,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 379,
                                                                                                            end: 383,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 379,
                                                                                                    end: 383,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    AsmRegisterDeclaration {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 385,
                                                                                                end: 387,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 389,
                                                                                                            end: 394,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 389,
                                                                                                    end: 394,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    AsmRegisterDeclaration {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 396,
                                                                                                end: 398,
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
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 418,
                                                                                                end: 420,
                                                                                                as_str(): "lt",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        op_args: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 421,
                                                                                                    end: 423,
                                                                                                    as_str(): "r3",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 424,
                                                                                                    end: 426,
                                                                                                    as_str(): "r2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 427,
                                                                                                    end: 429,
                                                                                                    as_str(): "r1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 418,
                                                                                            end: 429,
                                                                                            as_str(): "lt r3 r2 r1",
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
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 447,
                                                                                            end: 449,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                return_type: Boolean,
                                                                                whole_block_span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 371,
                                                                                    end: 469,
                                                                                    as_str(): "asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                            ),
                                                                            start: 371,
                                                                            end: 469,
                                                                            as_str(): "asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 371,
                                                                    end: 469,
                                                                    as_str(): "asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 357,
                                                            end: 479,
                                                            as_str(): "{\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }",
                                                        },
                                                    },
                                                    parameters: [
                                                        FunctionParameter {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 330,
                                                                    end: 334,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_reference: false,
                                                            is_mutable: false,
                                                            mutability_span: Span {
                                                                src (ptr): 0x00007fb14c011bb0,
                                                                path: None,
                                                                start: 0,
                                                                end: 0,
                                                                as_str(): "",
                                                            },
                                                            type_info: SelfType,
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 330,
                                                                end: 334,
                                                                as_str(): "self",
                                                            },
                                                        },
                                                        FunctionParameter {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 336,
                                                                    end: 341,
                                                                    as_str(): "other",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_reference: false,
                                                            is_mutable: false,
                                                            mutability_span: Span {
                                                                src (ptr): 0x00007fb14c011bb0,
                                                                path: None,
                                                                start: 0,
                                                                end: 0,
                                                                as_str(): "",
                                                            },
                                                            type_info: SelfType,
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 343,
                                                                end: 347,
                                                                as_str(): "Self",
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 324,
                                                        end: 479,
                                                        as_str(): "fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }",
                                                    },
                                                    return_type: Boolean,
                                                    type_parameters: [],
                                                    return_type_span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 352,
                                                        end: 356,
                                                        as_str(): "bool",
                                                    },
                                                },
                                                FunctionDeclaration {
                                                    purity: Pure,
                                                    attributes: {},
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 491,
                                                            end: 493,
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
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 539,
                                                                                                end: 541,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 543,
                                                                                                            end: 547,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 543,
                                                                                                    end: 547,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    AsmRegisterDeclaration {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 549,
                                                                                                end: 551,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 553,
                                                                                                            end: 558,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 553,
                                                                                                    end: 558,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    AsmRegisterDeclaration {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 560,
                                                                                                end: 562,
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
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 582,
                                                                                                end: 584,
                                                                                                as_str(): "gt",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        op_args: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 585,
                                                                                                    end: 587,
                                                                                                    as_str(): "r3",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 588,
                                                                                                    end: 590,
                                                                                                    as_str(): "r2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 591,
                                                                                                    end: 593,
                                                                                                    as_str(): "r1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 582,
                                                                                            end: 593,
                                                                                            as_str(): "gt r3 r2 r1",
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
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 611,
                                                                                            end: 613,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                return_type: Boolean,
                                                                                whole_block_span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 535,
                                                                                    end: 633,
                                                                                    as_str(): "asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                            ),
                                                                            start: 535,
                                                                            end: 633,
                                                                            as_str(): "asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 535,
                                                                    end: 633,
                                                                    as_str(): "asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 521,
                                                            end: 643,
                                                            as_str(): "{\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }",
                                                        },
                                                    },
                                                    parameters: [
                                                        FunctionParameter {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 494,
                                                                    end: 498,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_reference: false,
                                                            is_mutable: false,
                                                            mutability_span: Span {
                                                                src (ptr): 0x00007fb14c011bb0,
                                                                path: None,
                                                                start: 0,
                                                                end: 0,
                                                                as_str(): "",
                                                            },
                                                            type_info: SelfType,
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 494,
                                                                end: 498,
                                                                as_str(): "self",
                                                            },
                                                        },
                                                        FunctionParameter {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 500,
                                                                    end: 505,
                                                                    as_str(): "other",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_reference: false,
                                                            is_mutable: false,
                                                            mutability_span: Span {
                                                                src (ptr): 0x00007fb14c011bb0,
                                                                path: None,
                                                                start: 0,
                                                                end: 0,
                                                                as_str(): "",
                                                            },
                                                            type_info: SelfType,
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 507,
                                                                end: 511,
                                                                as_str(): "Self",
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 488,
                                                        end: 643,
                                                        as_str(): "fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }",
                                                    },
                                                    return_type: Boolean,
                                                    type_parameters: [],
                                                    return_type_span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 516,
                                                        end: 520,
                                                        as_str(): "bool",
                                                    },
                                                },
                                            ],
                                            block_span: Span {
                                                src (ptr): 0x00007fb0fa919aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                ),
                                                start: 288,
                                                end: 649,
                                                as_str(): "impl core::ops::Ord for X {\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0fa919aa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                    ),
                                    start: 288,
                                    end: 649,
                                    as_str(): "impl core::ops::Ord for X {\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }",
                                },
                            },
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
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 668,
                                                                                    end: 670,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 668,
                                                                                    end: 670,
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
                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                ),
                                                                                start: 668,
                                                                                end: 670,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 668,
                                                                    end: 670,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
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
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 657,
                                                                                                    end: 658,
                                                                                                    as_str(): "X",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 657,
                                                                                                end: 658,
                                                                                                as_str(): "X",
                                                                                            },
                                                                                        },
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 660,
                                                                                                end: 661,
                                                                                                as_str(): "Y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 657,
                                                                                    end: 661,
                                                                                    as_str(): "X::Y",
                                                                                },
                                                                            },
                                                                            args: [
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            true,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 662,
                                                                                        end: 666,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                        ),
                                                                        start: 657,
                                                                        end: 667,
                                                                        as_str(): "X::Y(true)",
                                                                    },
                                                                },
                                                                Expression {
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
                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 671,
                                                                                                    end: 672,
                                                                                                    as_str(): "X",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 671,
                                                                                                end: 672,
                                                                                                as_str(): "X",
                                                                                            },
                                                                                        },
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 674,
                                                                                                end: 675,
                                                                                                as_str(): "Y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 671,
                                                                                    end: 675,
                                                                                    as_str(): "X::Y",
                                                                                },
                                                                            },
                                                                            args: [
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            true,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 676,
                                                                                        end: 680,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                        ),
                                                                        start: 671,
                                                                        end: 681,
                                                                        as_str(): "X::Y(true)",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 657,
                                                        end: 681,
                                                        as_str(): "X::Y(true) == X::Y(true)",
                                                    },
                                                },
                                                then: Expression {
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
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 692,
                                                                                        end: 693,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                ),
                                                                                start: 692,
                                                                                end: 693,
                                                                                as_str(): "a",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                        ),
                                                                        start: 692,
                                                                        end: 693,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 682,
                                                                end: 699,
                                                                as_str(): "{\n        a\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 682,
                                                        end: 699,
                                                        as_str(): "{\n        a\n    }",
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
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 715,
                                                                                            end: 716,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 715,
                                                                                    end: 716,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                            ),
                                                                            start: 715,
                                                                            end: 716,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 705,
                                                                    end: 722,
                                                                    as_str(): "{\n        a\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 705,
                                                            end: 722,
                                                            as_str(): "{\n        a\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0fa919aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                            ),
                                            start: 654,
                                            end: 722,
                                            as_str(): "if X::Y(true) == X::Y(true) {\n        a\n    } else {\n        a\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0fa919aa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                    ),
                                    start: 654,
                                    end: 722,
                                    as_str(): "if X::Y(true) == X::Y(true) {\n        a\n    } else {\n        a\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0fa919aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                            ),
                            start: 25,
                            end: 724,
                            as_str(): "{\n    let a = 255;\n\n    enum X {\n        Y: bool,\n    }\n\n    impl core::ops::Eq for X {\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n\n    impl core::ops::Ord for X {\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n    if X::Y(true) == X::Y(true) {\n        a\n    } else {\n        a\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0fa919aa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                        ),
                        start: 8,
                        end: 724,
                        as_str(): "fn main() -> u64 {\n    let a = 255;\n\n    enum X {\n        Y: bool,\n    }\n\n    impl core::ops::Eq for X {\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n\n    impl core::ops::Ord for X {\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n    if X::Y(true) == X::Y(true) {\n        a\n    } else {\n        a\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0fa919aa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                        ),
                        start: 21,
                        end: 24,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0fa919aa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
            ),
            start: 8,
            end: 724,
            as_str(): "fn main() -> u64 {\n    let a = 255;\n\n    enum X {\n        Y: bool,\n    }\n\n    impl core::ops::Eq for X {\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n\n    impl core::ops::Ord for X {\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n    if X::Y(true) == X::Y(true) {\n        a\n    } else {\n        a\n    }\n}",
        },
    },
]
