[
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 142,
                            end: 151,
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
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 161,
                                    end: 164,
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
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 165,
                                            end: 169,
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 165,
                                        end: 169,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 171,
                                            end: 176,
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 178,
                                        end: 181,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            return_type: SelfType,
                            return_type_span: Span {
                                src (ptr): 0x00007fe043ecf990,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                ),
                                start: 186,
                                end: 190,
                                as_str(): "Self",
                            },
                        },
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 199,
                                    end: 202,
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
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 203,
                                            end: 207,
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 203,
                                        end: 207,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 209,
                                            end: 214,
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 216,
                                        end: 219,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            return_type: SelfType,
                            return_type_span: Span {
                                src (ptr): 0x00007fe043ecf990,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                ),
                                start: 224,
                                end: 228,
                                as_str(): "Self",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Public,
                    span: Span {
                        src (ptr): 0x00007fe043ecf990,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                        ),
                        start: 132,
                        end: 232,
                        as_str(): "pub trait Shiftable {\n    fn lsh(self, other: u64) -> Self;\n    fn rsh(self, other: u64) -> Self;\n\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe043ecf990,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
            ),
            start: 132,
            end: 232,
            as_str(): "pub trait Shiftable {\n    fn lsh(self, other: u64) -> Self;\n    fn rsh(self, other: u64) -> Self;\n\n}",
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
                                src (ptr): 0x00007fe043ecf990,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                ),
                                start: 239,
                                end: 248,
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
                        src (ptr): 0x00007fe043ecf990,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                        ),
                        start: 253,
                        end: 256,
                        as_str(): "u64",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 266,
                                    end: 269,
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
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 310,
                                                                        end: 312,
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
                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                    ),
                                                                                    start: 315,
                                                                                    end: 319,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 315,
                                                                            end: 319,
                                                                            as_str(): "self",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 321,
                                                                        end: 323,
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
                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                    ),
                                                                                    start: 325,
                                                                                    end: 330,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 325,
                                                                            end: 330,
                                                                            as_str(): "other",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 332,
                                                                        end: 334,
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
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 350,
                                                                        end: 353,
                                                                        as_str(): "sll",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                op_args: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 354,
                                                                            end: 356,
                                                                            as_str(): "r3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 357,
                                                                            end: 359,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 360,
                                                                            end: 362,
                                                                            as_str(): "r2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 350,
                                                                    end: 362,
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
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 376,
                                                                    end: 378,
                                                                    as_str(): "r3",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 306,
                                                            end: 393,
                                                            as_str(): "asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 306,
                                                    end: 393,
                                                    as_str(): "asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 306,
                                            end: 393,
                                            as_str(): "asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 296,
                                    end: 399,
                                    as_str(): "{\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 270,
                                            end: 274,
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 270,
                                        end: 274,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 276,
                                            end: 281,
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 283,
                                        end: 286,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe043ecf990,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                ),
                                start: 263,
                                end: 399,
                                as_str(): "fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe043ecf990,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                ),
                                start: 291,
                                end: 295,
                                as_str(): "Self",
                            },
                        },
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 408,
                                    end: 411,
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
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 452,
                                                                        end: 454,
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
                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                    ),
                                                                                    start: 457,
                                                                                    end: 461,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 457,
                                                                            end: 461,
                                                                            as_str(): "self",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 463,
                                                                        end: 465,
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
                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                    ),
                                                                                    start: 467,
                                                                                    end: 472,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 467,
                                                                            end: 472,
                                                                            as_str(): "other",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            AsmRegisterDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 474,
                                                                        end: 476,
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
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 492,
                                                                        end: 495,
                                                                        as_str(): "srl",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                op_args: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 496,
                                                                            end: 498,
                                                                            as_str(): "r3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 499,
                                                                            end: 501,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 502,
                                                                            end: 504,
                                                                            as_str(): "r2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 492,
                                                                    end: 504,
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
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 518,
                                                                    end: 520,
                                                                    as_str(): "r3",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: UnsignedInteger(
                                                            SixtyFour,
                                                        ),
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 448,
                                                            end: 535,
                                                            as_str(): "asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 448,
                                                    end: 535,
                                                    as_str(): "asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 448,
                                            end: 535,
                                            as_str(): "asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 438,
                                    end: 541,
                                    as_str(): "{\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 412,
                                            end: 416,
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 412,
                                        end: 416,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 418,
                                            end: 423,
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 425,
                                        end: 428,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe043ecf990,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                ),
                                start: 405,
                                end: 541,
                                as_str(): "fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe043ecf990,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                ),
                                start: 433,
                                end: 437,
                                as_str(): "Self",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe043ecf990,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                        ),
                        start: 234,
                        end: 543,
                        as_str(): "impl Shiftable for u64 {\n    fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }\n\n    fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe043ecf990,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
            ),
            start: 234,
            end: 543,
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
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 548,
                            end: 551,
                            as_str(): "foo",
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
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 575,
                                                    end: 576,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 578,
                                                    end: 581,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        4,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 584,
                                                    end: 585,
                                                    as_str(): "4",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 567,
                                    end: 586,
                                    as_str(): "let mut x: u64 = 4;",
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
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 591,
                                                                    end: 592,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 591,
                                                            end: 592,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
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
                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                    ),
                                                                                    start: 597,
                                                                                    end: 598,
                                                                                    as_str(): "+",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                    ),
                                                                                    start: 597,
                                                                                    end: 598,
                                                                                    as_str(): "+",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "add",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                ),
                                                                                start: 597,
                                                                                end: 598,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 597,
                                                                    end: 598,
                                                                    as_str(): "+",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 595,
                                                                        end: 596,
                                                                        as_str(): "5",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 599,
                                                                        end: 600,
                                                                        as_str(): "2",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 595,
                                                        end: 600,
                                                        as_str(): "5 + 2",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 591,
                                            end: 600,
                                            as_str(): "x = 5 + 2",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 591,
                                    end: 600,
                                    as_str(): "x = 5 + 2",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 606,
                                                    end: 607,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 606,
                                            end: 607,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 606,
                                    end: 607,
                                    as_str(): "x",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 561,
                            end: 609,
                            as_str(): "{\n    let mut x: u64 = 4;\n    x = 5 + 2;\n    x\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe043ecf990,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                        ),
                        start: 545,
                        end: 609,
                        as_str(): "fn foo() -> u64 {\n    let mut x: u64 = 4;\n    x = 5 + 2;\n    x\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe043ecf990,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                        ),
                        start: 557,
                        end: 560,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe043ecf990,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
            ),
            start: 545,
            end: 609,
            as_str(): "fn foo() -> u64 {\n    let mut x: u64 = 4;\n    x = 5 + 2;\n    x\n}",
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
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 614,
                            end: 618,
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
                                                                src (ptr): 0x00007fe043ecf990,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                ),
                                                                start: 632,
                                                                end: 635,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 632,
                                                        end: 635,
                                                        as_str(): "foo",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 632,
                                            end: 637,
                                            as_str(): "foo()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 632,
                                    end: 637,
                                    as_str(): "foo()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 628,
                            end: 639,
                            as_str(): "{\n  foo()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe043ecf990,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                        ),
                        start: 611,
                        end: 639,
                        as_str(): "fn main() -> u64 {\n  foo()\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe043ecf990,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                        ),
                        start: 624,
                        end: 627,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe043ecf990,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
            ),
            start: 611,
            end: 639,
            as_str(): "fn main() -> u64 {\n  foo()\n}",
        },
    },
]
