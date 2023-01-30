[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 18,
                            end: 24,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 26,
                            end: 32,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 9,
            end: 33,
            as_str(): "use std::assert::assert;",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 41,
                            end: 46,
                            as_str(): "MyAdd",
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
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 56,
                                    end: 62,
                                    as_str(): "my_add",
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
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 63,
                                            end: 67,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 63,
                                        end: 67,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 69,
                                            end: 74,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 76,
                                        end: 80,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            return_type: SelfType,
                            return_type_span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 85,
                                end: 89,
                                as_str(): "Self",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 35,
                        end: 92,
                        as_str(): "trait MyAdd {\n    fn my_add(self, other: Self) -> Self;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 35,
            end: 92,
            as_str(): "trait MyAdd {\n    fn my_add(self, other: Self) -> Self;\n}",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 100,
                            end: 105,
                            as_str(): "MyMul",
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
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 115,
                                    end: 121,
                                    as_str(): "my_mul",
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
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 122,
                                            end: 126,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 122,
                                        end: 126,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 133,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 135,
                                        end: 139,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            return_type: SelfType,
                            return_type_span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 144,
                                end: 148,
                                as_str(): "Self",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 94,
                        end: 151,
                        as_str(): "trait MyMul {\n    fn my_mul(self, other: Self) -> Self;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 94,
            end: 151,
            as_str(): "trait MyMul {\n    fn my_mul(self, other: Self) -> Self;\n}",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 159,
                            end: 165,
                            as_str(): "MyMath",
                        },
                        is_raw_ident: false,
                    },
                    type_parameters: [],
                    attributes: {},
                    interface_surface: [],
                    methods: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 195,
                                    end: 204,
                                    as_str(): "my_double",
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
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 234,
                                                                        end: 240,
                                                                        as_str(): "my_add",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 234,
                                                                end: 240,
                                                                as_str(): "my_add",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 229,
                                                                            end: 233,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 229,
                                                                    end: 233,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 241,
                                                                            end: 245,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 241,
                                                                    end: 245,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 229,
                                                    end: 246,
                                                    as_str(): "self.my_add(self)",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 229,
                                            end: 246,
                                            as_str(): "self.my_add(self)",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 219,
                                    end: 252,
                                    as_str(): "{\n        self.my_add(self)\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 205,
                                            end: 209,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 205,
                                        end: 209,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 192,
                                end: 252,
                                as_str(): "fn my_double(self) -> Self {\n        self.my_add(self)\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 214,
                                end: 218,
                                as_str(): "Self",
                            },
                        },
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 261,
                                    end: 267,
                                    as_str(): "my_exp",
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
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 297,
                                                                        end: 303,
                                                                        as_str(): "my_mul",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 297,
                                                                end: 303,
                                                                as_str(): "my_mul",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 292,
                                                                            end: 296,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 292,
                                                                    end: 296,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 304,
                                                                            end: 308,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 304,
                                                                    end: 308,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 292,
                                                    end: 309,
                                                    as_str(): "self.my_mul(self)",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 292,
                                            end: 309,
                                            as_str(): "self.my_mul(self)",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 282,
                                    end: 315,
                                    as_str(): "{\n        self.my_mul(self)\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 268,
                                            end: 272,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 268,
                                        end: 272,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 258,
                                end: 315,
                                as_str(): "fn my_exp(self) -> Self {\n        self.my_mul(self)\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 277,
                                end: 281,
                                as_str(): "Self",
                            },
                        },
                    ],
                    supertraits: [
                        Supertrait {
                            name: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 167,
                                        end: 172,
                                        as_str(): "MyAdd",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                        },
                        Supertrait {
                            name: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 175,
                                        end: 180,
                                        as_str(): "MyMul",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                        },
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 153,
                        end: 317,
                        as_str(): "trait MyMath: MyAdd + MyMul {\n\n} {\n    fn my_double(self) -> Self {\n        self.my_add(self)\n    }\n\n    fn my_exp(self) -> Self {\n        self.my_mul(self)\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 153,
            end: 317,
            as_str(): "trait MyMath: MyAdd + MyMul {\n\n} {\n    fn my_double(self) -> Self {\n        self.my_add(self)\n    }\n\n    fn my_exp(self) -> Self {\n        self.my_mul(self)\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 326,
                            end: 330,
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
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 337,
                                    end: 342,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 337,
                                end: 347,
                                as_str(): "value: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 344,
                                end: 347,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 319,
                        end: 350,
                        as_str(): "struct Data {\n    value: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 319,
            end: 350,
            as_str(): "struct Data {\n    value: u64,\n}",
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
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 357,
                                end: 362,
                                as_str(): "MyAdd",
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
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 367,
                                end: 371,
                                as_str(): "Data",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 367,
                        end: 371,
                        as_str(): "Data",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 381,
                                    end: 387,
                                    as_str(): "my_add",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
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
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 425,
                                                                        end: 429,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 425,
                                                                end: 429,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 444,
                                                                        end: 449,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
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
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 462,
                                                                                                    end: 463,
                                                                                                    as_str(): "+",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 462,
                                                                                                    end: 463,
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
                                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                ),
                                                                                                start: 462,
                                                                                                end: 463,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                    ),
                                                                                    start: 462,
                                                                                    end: 463,
                                                                                    as_str(): "+",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                            ),
                                                                                                            start: 451,
                                                                                                            end: 455,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 451,
                                                                                                    end: 455,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 456,
                                                                                                    end: 461,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 451,
                                                                                        end: 461,
                                                                                        as_str(): "self.value",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                            ),
                                                                                                            start: 464,
                                                                                                            end: 469,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 464,
                                                                                                    end: 469,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 470,
                                                                                                    end: 475,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 464,
                                                                                        end: 475,
                                                                                        as_str(): "other.value",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 451,
                                                                        end: 475,
                                                                        as_str(): "self.value + other.value",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 444,
                                                                    end: 475,
                                                                    as_str(): "value: self.value + other.value",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 425,
                                                    end: 485,
                                                    as_str(): "Data {\n            value: self.value + other.value\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 425,
                                            end: 485,
                                            as_str(): "Data {\n            value: self.value + other.value\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 415,
                                    end: 491,
                                    as_str(): "{\n        Data {\n            value: self.value + other.value\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 388,
                                            end: 392,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 388,
                                        end: 392,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 394,
                                            end: 399,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 401,
                                        end: 405,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 378,
                                end: 491,
                                as_str(): "fn my_add(self, other: Self) -> Self {\n        Data {\n            value: self.value + other.value\n        }\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 410,
                                end: 414,
                                as_str(): "Self",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 352,
                        end: 493,
                        as_str(): "impl MyAdd for Data {\n    fn my_add(self, other: Self) -> Self {\n        Data {\n            value: self.value + other.value\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 352,
            end: 493,
            as_str(): "impl MyAdd for Data {\n    fn my_add(self, other: Self) -> Self {\n        Data {\n            value: self.value + other.value\n        }\n    }\n}",
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
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 500,
                                end: 505,
                                as_str(): "MyMul",
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
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 510,
                                end: 514,
                                as_str(): "Data",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 510,
                        end: 514,
                        as_str(): "Data",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 524,
                                    end: 530,
                                    as_str(): "my_mul",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
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
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 568,
                                                                        end: 572,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 568,
                                                                end: 572,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 587,
                                                                        end: 592,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
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
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 605,
                                                                                                    end: 606,
                                                                                                    as_str(): "*",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 605,
                                                                                                    end: 606,
                                                                                                    as_str(): "*",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "multiply",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                ),
                                                                                                start: 605,
                                                                                                end: 606,
                                                                                                as_str(): "*",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                    ),
                                                                                    start: 605,
                                                                                    end: 606,
                                                                                    as_str(): "*",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                            ),
                                                                                                            start: 594,
                                                                                                            end: 598,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 594,
                                                                                                    end: 598,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 599,
                                                                                                    end: 604,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 594,
                                                                                        end: 604,
                                                                                        as_str(): "self.value",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                            ),
                                                                                                            start: 607,
                                                                                                            end: 612,
                                                                                                            as_str(): "other",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 607,
                                                                                                    end: 612,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 613,
                                                                                                    end: 618,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 607,
                                                                                        end: 618,
                                                                                        as_str(): "other.value",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 594,
                                                                        end: 618,
                                                                        as_str(): "self.value * other.value",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 587,
                                                                    end: 618,
                                                                    as_str(): "value: self.value * other.value",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 568,
                                                    end: 628,
                                                    as_str(): "Data {\n            value: self.value * other.value\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 568,
                                            end: 628,
                                            as_str(): "Data {\n            value: self.value * other.value\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 558,
                                    end: 634,
                                    as_str(): "{\n        Data {\n            value: self.value * other.value\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 531,
                                            end: 535,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 531,
                                        end: 535,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 537,
                                            end: 542,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 544,
                                        end: 548,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 521,
                                end: 634,
                                as_str(): "fn my_mul(self, other: Self) -> Self {\n        Data {\n            value: self.value * other.value\n        }\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 553,
                                end: 557,
                                as_str(): "Self",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 495,
                        end: 636,
                        as_str(): "impl MyMul for Data {\n    fn my_mul(self, other: Self) -> Self {\n        Data {\n            value: self.value * other.value\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 495,
            end: 636,
            as_str(): "impl MyMul for Data {\n    fn my_mul(self, other: Self) -> Self {\n        Data {\n            value: self.value * other.value\n        }\n    }\n}",
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
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 643,
                                end: 649,
                                as_str(): "MyMath",
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
                                src (ptr): 0x00007fe02ab473d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                ),
                                start: 654,
                                end: 658,
                                as_str(): "Data",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 654,
                        end: 658,
                        as_str(): "Data",
                    },
                    functions: [],
                    block_span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 638,
                        end: 661,
                        as_str(): "impl MyMath for Data {}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 638,
            end: 661,
            as_str(): "impl MyMath for Data {}",
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
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 666,
                            end: 670,
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
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 691,
                                                    end: 692,
                                                    as_str(): "a",
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
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 695,
                                                                        end: 699,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 695,
                                                                end: 699,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 710,
                                                                        end: 715,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 717,
                                                                        end: 721,
                                                                        as_str(): "3u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 710,
                                                                    end: 721,
                                                                    as_str(): "value: 3u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 695,
                                                    end: 727,
                                                    as_str(): "Data {\n        value: 3u64\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 687,
                                    end: 728,
                                    as_str(): "let a = Data {\n        value: 3u64\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 737,
                                                    end: 738,
                                                    as_str(): "b",
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
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 743,
                                                                        end: 749,
                                                                        as_str(): "my_exp",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 743,
                                                                end: 749,
                                                                as_str(): "my_exp",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 741,
                                                                            end: 742,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 741,
                                                                    end: 742,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 741,
                                                    end: 751,
                                                    as_str(): "a.my_exp()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 733,
                                    end: 752,
                                    as_str(): "let b = a.my_exp();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 761,
                                                    end: 762,
                                                    as_str(): "c",
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
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 767,
                                                                        end: 776,
                                                                        as_str(): "my_double",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 767,
                                                                end: 776,
                                                                as_str(): "my_double",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 765,
                                                                            end: 766,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 765,
                                                                    end: 766,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 765,
                                                    end: 778,
                                                    as_str(): "b.my_double()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 757,
                                    end: 779,
                                    as_str(): "let c = b.my_double();",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 784,
                                                                end: 790,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 784,
                                                        end: 790,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
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
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 799,
                                                                                        end: 801,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 799,
                                                                                        end: 801,
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
                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                    ),
                                                                                    start: 799,
                                                                                    end: 801,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 799,
                                                                        end: 801,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                ),
                                                                                                start: 791,
                                                                                                end: 792,
                                                                                                as_str(): "c",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 791,
                                                                                        end: 792,
                                                                                        as_str(): "c",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 793,
                                                                                        end: 798,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 791,
                                                                            end: 798,
                                                                            as_str(): "c.value",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                18,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 802,
                                                                            end: 804,
                                                                            as_str(): "18",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 791,
                                                            end: 804,
                                                            as_str(): "c.value == 18",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 784,
                                            end: 805,
                                            as_str(): "assert(c.value == 18)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 784,
                                    end: 805,
                                    as_str(): "assert(c.value == 18)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 812,
                                            end: 816,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 812,
                                    end: 816,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 681,
                            end: 818,
                            as_str(): "{\n    let a = Data {\n        value: 3u64\n    };\n    let b = a.my_exp();\n    let c = b.my_double();\n    assert(c.value == 18);\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 663,
                        end: 818,
                        as_str(): "fn main() -> bool {\n    let a = Data {\n        value: 3u64\n    };\n    let b = a.my_exp();\n    let c = b.my_double();\n    assert(c.value == 18);\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 676,
                        end: 680,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 663,
            end: 818,
            as_str(): "fn main() -> bool {\n    let a = Data {\n        value: 3u64\n    };\n    let b = a.my_exp();\n    let c = b.my_double();\n    assert(c.value == 18);\n\n    true\n}",
        },
    },
]
