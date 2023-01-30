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
                            src (ptr): 0x00007fe0fc1c14b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                            ),
                            start: 12,
                            end: 16,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc1c14b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                            ),
                            start: 19,
                            end: 21,
                            as_str(): "{}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fc1c14b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                        ),
                        start: 9,
                        end: 21,
                        as_str(): "fn main() {}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc1c14b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                        ),
                        start: 9,
                        end: 18,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc1c14b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
            ),
            start: 9,
            end: 21,
            as_str(): "fn main() {}",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc1c14b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                            ),
                            start: 33,
                            end: 42,
                            as_str(): "Shiftable",
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
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 52,
                                    end: 55,
                                    as_str(): "lsh",
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
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 56,
                                            end: 60,
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
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 56,
                                        end: 60,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 62,
                                            end: 67,
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
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 73,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            return_type: SelfType,
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 78,
                                end: 82,
                                as_str(): "Self",
                            },
                        },
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 91,
                                    end: 94,
                                    as_str(): "rsh",
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
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 95,
                                            end: 99,
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
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 99,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 101,
                                            end: 106,
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
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 108,
                                        end: 112,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            return_type: SelfType,
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 117,
                                end: 121,
                                as_str(): "Self",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Public,
                    span: Span {
                        src (ptr): 0x00007fe0fc1c14b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                        ),
                        start: 23,
                        end: 124,
                        as_str(): "pub trait Shiftable {\n    fn lsh(self, other: Self) -> Self;\n    fn rsh(self, other: Self) -> Self;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc1c14b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
            ),
            start: 23,
            end: 124,
            as_str(): "pub trait Shiftable {\n    fn lsh(self, other: Self) -> Self;\n    fn rsh(self, other: Self) -> Self;\n}",
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
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 131,
                                end: 140,
                                as_str(): "Shiftable",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fc1c14b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                        ),
                        start: 145,
                        end: 148,
                        as_str(): "u64",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 158,
                                    end: 161,
                                    as_str(): "lsh",
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
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 202,
                                                                        end: 204,
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
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 207,
                                                                                    end: 211,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 207,
                                                                            end: 211,
                                                                            as_str(): "self",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 213,
                                                                        end: 215,
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
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 217,
                                                                                    end: 222,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 217,
                                                                            end: 222,
                                                                            as_str(): "other",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 224,
                                                                        end: 226,
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
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 242,
                                                                        end: 245,
                                                                        as_str(): "sll",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                op_args: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 246,
                                                                            end: 248,
                                                                            as_str(): "r3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 249,
                                                                            end: 251,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 252,
                                                                            end: 254,
                                                                            as_str(): "r2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 242,
                                                                    end: 254,
                                                                    as_str(): "sll r3 r1 r2",
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
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 268,
                                                                    end: 270,
                                                                    as_str(): "r3",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 198,
                                                            end: 285,
                                                            as_str(): "asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 198,
                                                    end: 285,
                                                    as_str(): "asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 198,
                                            end: 285,
                                            as_str(): "asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 291,
                                    as_str(): "{\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 162,
                                            end: 166,
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
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 162,
                                        end: 166,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 168,
                                            end: 173,
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
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 175,
                                        end: 178,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 155,
                                end: 291,
                                as_str(): "fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 183,
                                end: 187,
                                as_str(): "Self",
                            },
                        },
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 300,
                                    end: 303,
                                    as_str(): "rsh",
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
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 344,
                                                                        end: 346,
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
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 349,
                                                                                    end: 353,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 349,
                                                                            end: 353,
                                                                            as_str(): "self",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 355,
                                                                        end: 357,
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
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 359,
                                                                                    end: 364,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 359,
                                                                            end: 364,
                                                                            as_str(): "other",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 366,
                                                                        end: 368,
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
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 384,
                                                                        end: 387,
                                                                        as_str(): "srl",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                op_args: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 388,
                                                                            end: 390,
                                                                            as_str(): "r3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 391,
                                                                            end: 393,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 394,
                                                                            end: 396,
                                                                            as_str(): "r2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 384,
                                                                    end: 396,
                                                                    as_str(): "srl r3 r1 r2",
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
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 410,
                                                                    end: 412,
                                                                    as_str(): "r3",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 427,
                                                            as_str(): "asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 340,
                                                    end: 427,
                                                    as_str(): "asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 340,
                                            end: 427,
                                            as_str(): "asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 330,
                                    end: 433,
                                    as_str(): "{\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 304,
                                            end: 308,
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
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 304,
                                        end: 308,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 310,
                                            end: 315,
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
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 317,
                                        end: 320,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 297,
                                end: 433,
                                as_str(): "fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 325,
                                end: 329,
                                as_str(): "Self",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fc1c14b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                        ),
                        start: 126,
                        end: 435,
                        as_str(): "impl Shiftable for u64 {\n    fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }\n\n    fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc1c14b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
            ),
            start: 126,
            end: 435,
            as_str(): "impl Shiftable for u64 {\n    fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }\n\n    fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }\n}",
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
                            src (ptr): 0x00007fe0fc1c14b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                            ),
                            start: 440,
                            end: 444,
                            as_str(): "sqrt",
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
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 520,
                                                    end: 521,
                                                    as_str(): "z",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 522,
                                                    end: 525,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 528,
                                                    end: 529,
                                                    as_str(): "1",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 512,
                                    end: 530,
                                    as_str(): "let mut z:u64 = 1;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 547,
                                                    end: 548,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 549,
                                                    end: 552,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 555,
                                                            end: 560,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 555,
                                                    end: 560,
                                                    as_str(): "value",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 539,
                                    end: 561,
                                    as_str(): "let mut y:u64 = value;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 574,
                                                    end: 575,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 578,
                                                    end: 582,
                                                    as_str(): "true",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 570,
                                    end: 583,
                                    as_str(): "let x = true;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 595,
                                                                end: 596,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                        ),
                                                        start: 595,
                                                        end: 596,
                                                        as_str(): "x",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Reassignment(
                                                                                ReassignmentExpression {
                                                                                    lhs: VariableExpression(
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                        ),
                                                                                                        start: 611,
                                                                                                        end: 612,
                                                                                                        as_str(): "y",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 611,
                                                                                                end: 612,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    rhs: Expression {
                                                                                        kind: MethodApplication(
                                                                                            MethodApplicationExpression {
                                                                                                method_name_binding: TypeBinding {
                                                                                                    inner: FromModule {
                                                                                                        method_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                ),
                                                                                                                start: 617,
                                                                                                                end: 620,
                                                                                                                as_str(): "rsh",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                        ),
                                                                                                        start: 617,
                                                                                                        end: 620,
                                                                                                        as_str(): "rsh",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 615,
                                                                                                                    end: 616,
                                                                                                                    as_str(): "y",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                            ),
                                                                                                            start: 615,
                                                                                                            end: 616,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                32,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                            ),
                                                                                                            start: 621,
                                                                                                            end: 623,
                                                                                                            as_str(): "32",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 615,
                                                                                            end: 624,
                                                                                            as_str(): "y.rsh(32)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 611,
                                                                                end: 624,
                                                                                as_str(): "y = y.rsh(32)",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 611,
                                                                        end: 624,
                                                                        as_str(): "y = y.rsh(32)",
                                                                    },
                                                                },
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Reassignment(
                                                                                ReassignmentExpression {
                                                                                    lhs: VariableExpression(
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                        ),
                                                                                                        start: 638,
                                                                                                        end: 639,
                                                                                                        as_str(): "z",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 638,
                                                                                                end: 639,
                                                                                                as_str(): "z",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    rhs: Expression {
                                                                                        kind: MethodApplication(
                                                                                            MethodApplicationExpression {
                                                                                                method_name_binding: TypeBinding {
                                                                                                    inner: FromModule {
                                                                                                        method_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                ),
                                                                                                                start: 644,
                                                                                                                end: 647,
                                                                                                                as_str(): "lsh",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                        ),
                                                                                                        start: 644,
                                                                                                        end: 647,
                                                                                                        as_str(): "lsh",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 642,
                                                                                                                    end: 643,
                                                                                                                    as_str(): "z",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                            ),
                                                                                                            start: 642,
                                                                                                            end: 643,
                                                                                                            as_str(): "z",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                16,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                            ),
                                                                                                            start: 648,
                                                                                                            end: 650,
                                                                                                            as_str(): "16",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 642,
                                                                                            end: 651,
                                                                                            as_str(): "z.lsh(16)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 638,
                                                                                end: 651,
                                                                                as_str(): "z = z.lsh(16)",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 638,
                                                                        end: 651,
                                                                        as_str(): "z = z.lsh(16)",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 597,
                                                                end: 662,
                                                                as_str(): "{\n            y = y.rsh(32);\n            z = z.lsh(16);\n        }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                        ),
                                                        start: 597,
                                                        end: 662,
                                                        as_str(): "{\n            y = y.rsh(32);\n            z = z.lsh(16);\n        }",
                                                    },
                                                },
                                                else: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 592,
                                            end: 662,
                                            as_str(): "if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 592,
                                    end: 662,
                                    as_str(): "if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        }",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 672,
                                                    end: 673,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 672,
                                            end: 673,
                                            as_str(): "y",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 672,
                                    end: 673,
                                    as_str(): "y",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc1c14b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                            ),
                            start: 502,
                            end: 675,
                            as_str(): "{\n        let mut z:u64 = 1;\n        let mut y:u64 = value;\n        let x = true;\n        if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        };\n        y\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 445,
                                    end: 449,
                                    as_str(): "gas_",
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 451,
                                end: 454,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 456,
                                    end: 463,
                                    as_str(): "amount_",
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 465,
                                end: 468,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 470,
                                    end: 475,
                                    as_str(): "coin_",
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
                            type_info: B256,
                            type_span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 477,
                                end: 481,
                                as_str(): "b256",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 483,
                                    end: 488,
                                    as_str(): "value",
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 490,
                                end: 493,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc1c14b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                        ),
                        start: 437,
                        end: 675,
                        as_str(): "fn sqrt(gas_: u64, amount_: u64, coin_: b256, value: u64)-> u64  {\n        let mut z:u64 = 1;\n        let mut y:u64 = value;\n        let x = true;\n        if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        };\n        y\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc1c14b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                        ),
                        start: 497,
                        end: 500,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc1c14b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
            ),
            start: 437,
            end: 675,
            as_str(): "fn sqrt(gas_: u64, amount_: u64, coin_: b256, value: u64)-> u64  {\n        let mut z:u64 = 1;\n        let mut y:u64 = value;\n        let x = true;\n        if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        };\n        y\n}",
        },
    },
]
