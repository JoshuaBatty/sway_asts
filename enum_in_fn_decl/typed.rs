TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        255,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31631,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Declaration(
                    EnumDeclaration(
                        DeclId(
                            13314,
                            Span {
                                src (ptr): 0x00007fb0fa919aa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                ),
                                start: 49,
                                end: 80,
                                as_str(): "enum X {\n        Y: bool,\n    }",
                            },
                        ),
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
            TyAstNode {
                content: Declaration(
                    ImplTrait(
                        DeclId(
                            13321,
                            Span {
                                src (ptr): 0x00007fb0fa919aa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                ),
                                start: 86,
                                end: 282,
                                as_str(): "impl core::ops::Eq for X {\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }",
                            },
                        ),
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
            TyAstNode {
                content: Declaration(
                    ImplTrait(
                        DeclId(
                            13324,
                            Span {
                                src (ptr): 0x00007fb0fa919aa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                ),
                                start: 288,
                                end: 649,
                                as_str(): "impl core::ops::Ord for X {\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }",
                            },
                        ),
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
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
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
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
                                            TyExpression {
                                                expression: EnumInstantiation {
                                                    enum_decl: TyEnumDeclaration {
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
                                                        type_parameters: [],
                                                        attributes: {},
                                                        variants: [
                                                            TyEnumVariant {
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
                                                                type_id: TypeId(
                                                                    71,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    71,
                                                                ),
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
                                                                attributes: {},
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
                                                    variant_name: BaseIdent {
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
                                                    tag: 0,
                                                    contents: Some(
                                                        TyExpression {
                                                            expression: Literal(
                                                                Boolean(
                                                                    true,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                71,
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
                                                    ),
                                                    enum_instantiation_span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 657,
                                                        end: 658,
                                                        as_str(): "X",
                                                    },
                                                    variant_instantiation_span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 660,
                                                        end: 661,
                                                        as_str(): "Y",
                                                    },
                                                    type_binding: TypeBinding {
                                                        inner: (),
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
                                                },
                                                return_type: TypeId(
                                                    31634,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0fa919aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                    ),
                                                    start: 660,
                                                    end: 661,
                                                    as_str(): "Y",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
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
                                            TyExpression {
                                                expression: EnumInstantiation {
                                                    enum_decl: TyEnumDeclaration {
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
                                                        type_parameters: [],
                                                        attributes: {},
                                                        variants: [
                                                            TyEnumVariant {
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
                                                                type_id: TypeId(
                                                                    71,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    71,
                                                                ),
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
                                                                attributes: {},
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
                                                    variant_name: BaseIdent {
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
                                                    tag: 0,
                                                    contents: Some(
                                                        TyExpression {
                                                            expression: Literal(
                                                                Boolean(
                                                                    true,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                71,
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
                                                    ),
                                                    enum_instantiation_span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 671,
                                                        end: 672,
                                                        as_str(): "X",
                                                    },
                                                    variant_instantiation_span: Span {
                                                        src (ptr): 0x00007fb0fa919aa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                        ),
                                                        start: 674,
                                                        end: 675,
                                                        as_str(): "Y",
                                                    },
                                                    type_binding: TypeBinding {
                                                        inner: (),
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
                                                },
                                                return_type: TypeId(
                                                    31634,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0fa919aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                    ),
                                                    start: 674,
                                                    end: 675,
                                                    as_str(): "Y",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13327,
                                        Span {
                                            src (ptr): 0x00007fb0fa919aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                            ),
                                            start: 121,
                                            end: 276,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
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
                                    ),
                                },
                                return_type: TypeId(
                                    71,
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
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: VariableExpression {
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
                                                            span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 692,
                                                                end: 693,
                                                                as_str(): "a",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
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
                                    },
                                ),
                                return_type: TypeId(
                                    21,
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
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: VariableExpression {
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
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 715,
                                                                    end: 716,
                                                                    as_str(): "a",
                                                                },
                                                                mutability: Immutable,
                                                            },
                                                            return_type: TypeId(
                                                                21,
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
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
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
                        return_type: TypeId(
                            21,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb0fa919aa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
        ),
        start: 8,
        end: 724,
        as_str(): "fn main() -> u64 {\n    let a = 255;\n\n    enum X {\n        Y: bool,\n    }\n\n    impl core::ops::Eq for X {\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n\n    impl core::ops::Ord for X {\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n    if X::Y(true) == X::Y(true) {\n        a\n    } else {\n        a\n    }\n}",
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
        src (ptr): 0x00007fb0fa919aa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
        ),
        start: 21,
        end: 24,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

