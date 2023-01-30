

TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 60,
            end: 61,
            as_str(): "A",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 68,
                    end: 69,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fe04ae0ff10,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                ),
                start: 68,
                end: 74,
                as_str(): "a: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe04ae0ff10,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                ),
                start: 71,
                end: 74,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 80,
                    end: 81,
                    as_str(): "b",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fe04ae0ff10,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                ),
                start: 80,
                end: 86,
                as_str(): "b: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe04ae0ff10,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                ),
                start: 83,
                end: 86,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe04ae0ff10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
        ),
        start: 53,
        end: 89,
        as_str(): "struct A {\n    a: u64,\n    b: u64,\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13320,
        Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 91,
            end: 379,
            as_str(): "impl A {\n    fn f(ref mut self) {\n        self.a = 42;\n        self.b = 77;\n    }\n\n    fn g(ref mut self, inc: u64) {\n        self.a = self.a + inc;\n        self.b = self.b + inc;\n    }\n\n    fn h(ref mut self) {\n        self = A {\n            a: 100,\n            b: 200,\n        }\n    }\n}",
        },
    ),
)
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 386,
            end: 387,
            as_str(): "E",
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
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 394,
                    end: 395,
                    as_str(): "X",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe04ae0ff10,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                ),
                start: 397,
                end: 400,
                as_str(): "u64",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe04ae0ff10,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                ),
                start: 394,
                end: 400,
                as_str(): "X: u64",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 406,
                    end: 407,
                    as_str(): "Y",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe04ae0ff10,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                ),
                start: 409,
                end: 412,
                as_str(): "u64",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe04ae0ff10,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                ),
                start: 406,
                end: 412,
                as_str(): "Y: u64",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe04ae0ff10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
        ),
        start: 381,
        end: 415,
        as_str(): "enum E {\n    X: u64,\n    Y: u64,\n}",
    },
    visibility: Private,
}
ImplTrait(
    DeclId(
        13329,
        Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 417,
            end: 590,
            as_str(): "impl E {\n    fn j(ref mut self, inc: u64) {\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 595,
            end: 599,
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
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 624,
                                    end: 625,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 60,
                                            end: 61,
                                            as_str(): "A",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 640,
                                                    end: 641,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 643,
                                                    end: 644,
                                                    as_str(): "0",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 654,
                                                    end: 655,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 657,
                                                    end: 658,
                                                    as_str(): "0",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 628,
                                        end: 629,
                                        as_str(): "A",
                                    },
                                },
                                return_type: TypeId(
                                    31631,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 628,
                                    end: 665,
                                    as_str(): "A {\n        a: 0,\n        b: 0,\n    }",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                31631,
                            ),
                            type_ascription: TypeId(
                                31745,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 616,
                    end: 666,
                    as_str(): "let mut a = A {\n        a: 0,\n        b: 0,\n    };",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 674,
                                        end: 675,
                                        as_str(): "f",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 121,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 624,
                                                    end: 625,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04ae0ff10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                ),
                                                start: 672,
                                                end: 673,
                                                as_str(): "a",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            31631,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 672,
                                            end: 673,
                                            as_str(): "a",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13330,
                                Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 104,
                                    end: 172,
                                    as_str(): "fn f(ref mut self) {\n        self.a = 42;\n        self.b = 77;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 674,
                                        end: 675,
                                        as_str(): "f",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31753,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 672,
                            end: 677,
                            as_str(): "a.f()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 672,
                    end: 677,
                    as_str(): "a.f()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 683,
                                        end: 689,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0515cace0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 694,
                                                            end: 696,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 694,
                                                            end: 696,
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
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 694,
                                                        end: 696,
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
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 624,
                                                                            end: 625,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 690,
                                                                        end: 691,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31631,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 690,
                                                                    end: 691,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 68,
                                                                        end: 69,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    21,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 74,
                                                                    as_str(): "a: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 74,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 692,
                                                                end: 693,
                                                                as_str(): "a",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31631,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 690,
                                                            end: 693,
                                                            as_str(): "a.a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 697,
                                                            end: 699,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13332,
                                                Span {
                                                    src (ptr): 0x00007fe05c40b7f0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 694,
                                                        end: 696,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 690,
                                            end: 699,
                                            as_str(): "a.a == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007fe0515cace0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 683,
                                        end: 689,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31760,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 683,
                            end: 700,
                            as_str(): "assert(a.a == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 683,
                    end: 700,
                    as_str(): "assert(a.a == 42)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 706,
                                        end: 712,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0515cace0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 717,
                                                            end: 719,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 717,
                                                            end: 719,
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
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 717,
                                                        end: 719,
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
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 624,
                                                                            end: 625,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 713,
                                                                        end: 714,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31631,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 713,
                                                                    end: 714,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 80,
                                                                        end: 81,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    21,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 80,
                                                                    end: 86,
                                                                    as_str(): "b: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 83,
                                                                    end: 86,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 715,
                                                                end: 716,
                                                                as_str(): "b",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31631,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 713,
                                                            end: 716,
                                                            as_str(): "a.b",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                77,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 720,
                                                            end: 722,
                                                            as_str(): "77",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13335,
                                                Span {
                                                    src (ptr): 0x00007fe05c40b7f0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 717,
                                                        end: 719,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 713,
                                            end: 722,
                                            as_str(): "a.b == 77",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13336,
                                Span {
                                    src (ptr): 0x00007fe0515cace0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 706,
                                        end: 712,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31767,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 706,
                            end: 723,
                            as_str(): "assert(a.b == 77)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 706,
                    end: 723,
                    as_str(): "assert(a.b == 77)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 732,
                                        end: 733,
                                        as_str(): "g",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 191,
                                            end: 195,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 624,
                                                    end: 625,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04ae0ff10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                ),
                                                start: 730,
                                                end: 731,
                                                as_str(): "a",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            31631,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 730,
                                            end: 731,
                                            as_str(): "a",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 197,
                                            end: 200,
                                            as_str(): "inc",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U64(
                                                1,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 734,
                                            end: 735,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 178,
                                    end: 276,
                                    as_str(): "fn g(ref mut self, inc: u64) {\n        self.a = self.a + inc;\n        self.b = self.b + inc;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 732,
                                        end: 733,
                                        as_str(): "g",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31772,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 730,
                            end: 736,
                            as_str(): "a.g(1)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 730,
                    end: 736,
                    as_str(): "a.g(1)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 742,
                                        end: 748,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0515cace0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 753,
                                                            end: 755,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 753,
                                                            end: 755,
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
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 753,
                                                        end: 755,
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
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 624,
                                                                            end: 625,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 749,
                                                                        end: 750,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31631,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 749,
                                                                    end: 750,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 68,
                                                                        end: 69,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    21,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 74,
                                                                    as_str(): "a: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 74,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 751,
                                                                end: 752,
                                                                as_str(): "a",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31631,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 749,
                                                            end: 752,
                                                            as_str(): "a.a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                43,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 756,
                                                            end: 758,
                                                            as_str(): "43",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13339,
                                                Span {
                                                    src (ptr): 0x00007fe05c40b7f0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 753,
                                                        end: 755,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 749,
                                            end: 758,
                                            as_str(): "a.a == 43",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13340,
                                Span {
                                    src (ptr): 0x00007fe0515cace0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 742,
                                        end: 748,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31779,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 742,
                            end: 759,
                            as_str(): "assert(a.a == 43)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 742,
                    end: 759,
                    as_str(): "assert(a.a == 43)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 765,
                                        end: 771,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0515cace0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 776,
                                                            end: 778,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 776,
                                                            end: 778,
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
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 776,
                                                        end: 778,
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
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 624,
                                                                            end: 625,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 772,
                                                                        end: 773,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31631,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 772,
                                                                    end: 773,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 80,
                                                                        end: 81,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    21,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 80,
                                                                    end: 86,
                                                                    as_str(): "b: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 83,
                                                                    end: 86,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 774,
                                                                end: 775,
                                                                as_str(): "b",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31631,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 772,
                                                            end: 775,
                                                            as_str(): "a.b",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                78,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 779,
                                                            end: 781,
                                                            as_str(): "78",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13342,
                                                Span {
                                                    src (ptr): 0x00007fe05c40b7f0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 776,
                                                        end: 778,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 772,
                                            end: 781,
                                            as_str(): "a.b == 78",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13343,
                                Span {
                                    src (ptr): 0x00007fe0515cace0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 765,
                                        end: 771,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31786,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 765,
                            end: 782,
                            as_str(): "assert(a.b == 78)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 765,
                    end: 782,
                    as_str(): "assert(a.b == 78)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 791,
                                        end: 792,
                                        as_str(): "h",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 295,
                                            end: 299,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 624,
                                                    end: 625,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04ae0ff10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                ),
                                                start: 789,
                                                end: 790,
                                                as_str(): "a",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            31631,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 789,
                                            end: 790,
                                            as_str(): "a",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13344,
                                Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 282,
                                    end: 377,
                                    as_str(): "fn h(ref mut self) {\n        self = A {\n            a: 100,\n            b: 200,\n        }\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 791,
                                        end: 792,
                                        as_str(): "h",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31789,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 789,
                            end: 794,
                            as_str(): "a.h()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 789,
                    end: 794,
                    as_str(): "a.h()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 800,
                                        end: 806,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0515cace0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 811,
                                                            end: 813,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 811,
                                                            end: 813,
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
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 811,
                                                        end: 813,
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
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 624,
                                                                            end: 625,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 807,
                                                                        end: 808,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31631,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 807,
                                                                    end: 808,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 68,
                                                                        end: 69,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    21,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 74,
                                                                    as_str(): "a: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 74,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 809,
                                                                end: 810,
                                                                as_str(): "a",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31631,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 807,
                                                            end: 810,
                                                            as_str(): "a.a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                100,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 814,
                                                            end: 817,
                                                            as_str(): "100",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13346,
                                                Span {
                                                    src (ptr): 0x00007fe05c40b7f0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 811,
                                                        end: 813,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 807,
                                            end: 817,
                                            as_str(): "a.a == 100",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13347,
                                Span {
                                    src (ptr): 0x00007fe0515cace0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 800,
                                        end: 806,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31796,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 800,
                            end: 818,
                            as_str(): "assert(a.a == 100)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 800,
                    end: 818,
                    as_str(): "assert(a.a == 100)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 824,
                                        end: 830,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0515cace0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 835,
                                                            end: 837,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 835,
                                                            end: 837,
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
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 835,
                                                        end: 837,
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
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 624,
                                                                            end: 625,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 831,
                                                                        end: 832,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31631,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 831,
                                                                    end: 832,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 80,
                                                                        end: 81,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    21,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 80,
                                                                    end: 86,
                                                                    as_str(): "b: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 83,
                                                                    end: 86,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 833,
                                                                end: 834,
                                                                as_str(): "b",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31631,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 831,
                                                            end: 834,
                                                            as_str(): "a.b",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c40b7f0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                200,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 838,
                                                            end: 841,
                                                            as_str(): "200",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13349,
                                                Span {
                                                    src (ptr): 0x00007fe05c40b7f0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 835,
                                                        end: 837,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 831,
                                            end: 841,
                                            as_str(): "a.b == 200",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13350,
                                Span {
                                    src (ptr): 0x00007fe0515cace0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 824,
                                        end: 830,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31803,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 824,
                            end: 842,
                            as_str(): "assert(a.b == 200)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 824,
                    end: 842,
                    as_str(): "assert(a.b == 200)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 857,
                                    end: 858,
                                    as_str(): "e",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe04ae0ff10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                ),
                                                start: 386,
                                                end: 387,
                                                as_str(): "E",
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
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 394,
                                                        end: 395,
                                                        as_str(): "X",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    21,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 397,
                                                    end: 400,
                                                    as_str(): "u64",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 394,
                                                    end: 400,
                                                    as_str(): "X: u64",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 406,
                                                        end: 407,
                                                        as_str(): "Y",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    21,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 409,
                                                    end: 412,
                                                    as_str(): "u64",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 406,
                                                    end: 412,
                                                    as_str(): "Y: u64",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 381,
                                            end: 415,
                                            as_str(): "enum E {\n    X: u64,\n    Y: u64,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 394,
                                            end: 395,
                                            as_str(): "X",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    42,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe04ae0ff10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                ),
                                                start: 866,
                                                end: 868,
                                                as_str(): "42",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 861,
                                        end: 862,
                                        as_str(): "E",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 864,
                                        end: 865,
                                        as_str(): "X",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 861,
                                            end: 865,
                                            as_str(): "E::X",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31700,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 864,
                                    end: 865,
                                    as_str(): "X",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                31700,
                            ),
                            type_ascription: TypeId(
                                31804,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 849,
                    end: 870,
                    as_str(): "let mut e = E::X(42);",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: CodeBlock(
                            TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Declaration(
                                            VariableDeclaration(
                                                TyVariableDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_2",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 881,
                                                            end: 882,
                                                            as_str(): "e",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 857,
                                                                    end: 858,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 881,
                                                                end: 882,
                                                                as_str(): "e",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            31700,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 881,
                                                            end: 882,
                                                            as_str(): "e",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31700,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31808,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 875,
                                            end: 938,
                                            as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: IfExp {
                                                    condition: TyExpression {
                                                        expression: LazyOperator {
                                                            op: And,
                                                            lhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 881,
                                                                                    end: 882,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 881,
                                                                                    end: 882,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 881,
                                                                                end: 882,
                                                                                as_str(): "e",
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
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3022,
                                                                                    end: 3026,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: EnumTag {
                                                                                    exp: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_2",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 881,
                                                                                                    end: 882,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 881,
                                                                                                end: 882,
                                                                                                as_str(): "e",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31700,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 881,
                                                                                            end: 882,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 881,
                                                                                    end: 882,
                                                                                    as_str(): "e",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3028,
                                                                                    end: 3033,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    U64(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 881,
                                                                                    end: 882,
                                                                                    as_str(): "e",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13355,
                                                                        Span {
                                                                            src (ptr): 0x00007fe05c40b7f0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3016,
                                                                            end: 3082,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 881,
                                                                    end: 882,
                                                                    as_str(): "e",
                                                                },
                                                            },
                                                            rhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 893,
                                                                                    end: 901,
                                                                                    as_str(): "E::X(42)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 893,
                                                                                    end: 901,
                                                                                    as_str(): "E::X(42)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 893,
                                                                                end: 901,
                                                                                as_str(): "E::X(42)",
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
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3022,
                                                                                    end: 3026,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: UnsafeDowncast {
                                                                                    exp: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_2",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 881,
                                                                                                    end: 882,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 881,
                                                                                                end: 882,
                                                                                                as_str(): "e",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31700,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 881,
                                                                                            end: 882,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                    },
                                                                                    variant: TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 394,
                                                                                                end: 395,
                                                                                                as_str(): "X",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 397,
                                                                                            end: 400,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 394,
                                                                                            end: 400,
                                                                                            as_str(): "X: u64",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 893,
                                                                                    end: 901,
                                                                                    as_str(): "E::X(42)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3028,
                                                                                    end: 3033,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    Numeric(
                                                                                        42,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 898,
                                                                                    end: 900,
                                                                                    as_str(): "42",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13354,
                                                                        Span {
                                                                            src (ptr): 0x00007fe05c40b7f0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3016,
                                                                            end: 3082,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 893,
                                                                    end: 901,
                                                                    as_str(): "E::X(42)",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 881,
                                                            end: 901,
                                                            as_str(): "e {\n        E::X(42)",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31813,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 905,
                                                            end: 907,
                                                            as_str(): "{}",
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
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 922,
                                                                                                    end: 928,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        contract_call_params: {},
                                                                                        arguments: [
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe050cc3b50,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                                        ),
                                                                                                        start: 582,
                                                                                                        end: 586,
                                                                                                        as_str(): "code",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: Literal(
                                                                                                        U64(
                                                                                                            0,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 929,
                                                                                                        end: 930,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13353,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe050cc3b50,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                                ),
                                                                                                start: 568,
                                                                                                end: 615,
                                                                                                as_str(): "pub fn revert(code: u64) {\n    __revert(code)\n}",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: Some(
                                                                                            TypeBinding {
                                                                                                inner: (),
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 922,
                                                                                                    end: 928,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        31819,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 922,
                                                                                        end: 931,
                                                                                        as_str(): "revert(0)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 922,
                                                                                end: 931,
                                                                                as_str(): "revert(0)",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31819,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 922,
                                                                end: 931,
                                                                as_str(): "revert(0)",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    31820,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 905,
                                                    end: 907,
                                                    as_str(): "{}",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 875,
                                            end: 938,
                                            as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            31821,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 875,
                            end: 938,
                            as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 875,
                    end: 938,
                    as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 951,
                                        end: 952,
                                        as_str(): "j",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 443,
                                            end: 447,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 857,
                                                    end: 858,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04ae0ff10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                ),
                                                start: 949,
                                                end: 950,
                                                as_str(): "e",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            31700,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 949,
                                            end: 950,
                                            as_str(): "e",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 449,
                                            end: 452,
                                            as_str(): "inc",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U64(
                                                4,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 953,
                                            end: 954,
                                            as_str(): "4",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13356,
                                Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 430,
                                    end: 588,
                                    as_str(): "fn j(ref mut self, inc: u64) {\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 951,
                                        end: 952,
                                        as_str(): "j",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31826,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 949,
                            end: 955,
                            as_str(): "e.j(4)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 949,
                    end: 955,
                    as_str(): "e.j(4)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: CodeBlock(
                            TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Declaration(
                                            VariableDeclaration(
                                                TyVariableDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_3",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 967,
                                                            end: 968,
                                                            as_str(): "e",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 857,
                                                                    end: 858,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 967,
                                                                end: 968,
                                                                as_str(): "e",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            31700,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 967,
                                                            end: 968,
                                                            as_str(): "e",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31700,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31828,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 961,
                                            end: 1024,
                                            as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: IfExp {
                                                    condition: TyExpression {
                                                        expression: LazyOperator {
                                                            op: And,
                                                            lhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 967,
                                                                                    end: 968,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 967,
                                                                                    end: 968,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 967,
                                                                                end: 968,
                                                                                as_str(): "e",
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
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3022,
                                                                                    end: 3026,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: EnumTag {
                                                                                    exp: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_3",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 967,
                                                                                                    end: 968,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 967,
                                                                                                end: 968,
                                                                                                as_str(): "e",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31700,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 967,
                                                                                            end: 968,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 967,
                                                                                    end: 968,
                                                                                    as_str(): "e",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3028,
                                                                                    end: 3033,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    U64(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 967,
                                                                                    end: 968,
                                                                                    as_str(): "e",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13360,
                                                                        Span {
                                                                            src (ptr): 0x00007fe05c40b7f0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3016,
                                                                            end: 3082,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 967,
                                                                    end: 968,
                                                                    as_str(): "e",
                                                                },
                                                            },
                                                            rhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 979,
                                                                                    end: 987,
                                                                                    as_str(): "E::Y(46)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 979,
                                                                                    end: 987,
                                                                                    as_str(): "E::Y(46)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 979,
                                                                                end: 987,
                                                                                as_str(): "E::Y(46)",
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
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3022,
                                                                                    end: 3026,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: UnsafeDowncast {
                                                                                    exp: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_3",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 967,
                                                                                                    end: 968,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 967,
                                                                                                end: 968,
                                                                                                as_str(): "e",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31700,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 967,
                                                                                            end: 968,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                    },
                                                                                    variant: TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 406,
                                                                                                end: 407,
                                                                                                as_str(): "Y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 409,
                                                                                            end: 412,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 406,
                                                                                            end: 412,
                                                                                            as_str(): "Y: u64",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 979,
                                                                                    end: 987,
                                                                                    as_str(): "E::Y(46)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3028,
                                                                                    end: 3033,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    Numeric(
                                                                                        46,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 984,
                                                                                    end: 986,
                                                                                    as_str(): "46",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13359,
                                                                        Span {
                                                                            src (ptr): 0x00007fe05c40b7f0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3016,
                                                                            end: 3082,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 979,
                                                                    end: 987,
                                                                    as_str(): "E::Y(46)",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 967,
                                                            end: 987,
                                                            as_str(): "e {\n        E::Y(46)",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31833,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 991,
                                                            end: 993,
                                                            as_str(): "{}",
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
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1008,
                                                                                                    end: 1014,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        contract_call_params: {},
                                                                                        arguments: [
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe050cc3b50,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                                        ),
                                                                                                        start: 582,
                                                                                                        end: 586,
                                                                                                        as_str(): "code",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: Literal(
                                                                                                        U64(
                                                                                                            0,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1015,
                                                                                                        end: 1016,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13358,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe050cc3b50,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                                ),
                                                                                                start: 568,
                                                                                                end: 615,
                                                                                                as_str(): "pub fn revert(code: u64) {\n    __revert(code)\n}",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: Some(
                                                                                            TypeBinding {
                                                                                                inner: (),
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1008,
                                                                                                    end: 1014,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        31839,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1008,
                                                                                        end: 1017,
                                                                                        as_str(): "revert(0)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1008,
                                                                                end: 1017,
                                                                                as_str(): "revert(0)",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31839,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 1008,
                                                                end: 1017,
                                                                as_str(): "revert(0)",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    31840,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 991,
                                                    end: 993,
                                                    as_str(): "{}",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 961,
                                            end: 1024,
                                            as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            31841,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 961,
                            end: 1024,
                            as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 961,
                    end: 1024,
                    as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 1033,
                                        end: 1034,
                                        as_str(): "j",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 443,
                                            end: 447,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 857,
                                                    end: 858,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04ae0ff10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                ),
                                                start: 1031,
                                                end: 1032,
                                                as_str(): "e",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            31700,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 1031,
                                            end: 1032,
                                            as_str(): "e",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 449,
                                            end: 452,
                                            as_str(): "inc",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U64(
                                                5,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 1035,
                                            end: 1036,
                                            as_str(): "5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 430,
                                    end: 588,
                                    as_str(): "fn j(ref mut self, inc: u64) {\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 1033,
                                        end: 1034,
                                        as_str(): "j",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31846,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 1031,
                            end: 1037,
                            as_str(): "e.j(5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 1031,
                    end: 1037,
                    as_str(): "e.j(5)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: CodeBlock(
                            TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Declaration(
                                            VariableDeclaration(
                                                TyVariableDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_4",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1049,
                                                            end: 1050,
                                                            as_str(): "e",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 857,
                                                                    end: 858,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 1049,
                                                                end: 1050,
                                                                as_str(): "e",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            31700,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1049,
                                                            end: 1050,
                                                            as_str(): "e",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31700,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31848,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 1043,
                                            end: 1106,
                                            as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: IfExp {
                                                    condition: TyExpression {
                                                        expression: LazyOperator {
                                                            op: And,
                                                            lhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1049,
                                                                                    end: 1050,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1049,
                                                                                    end: 1050,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1049,
                                                                                end: 1050,
                                                                                as_str(): "e",
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
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3022,
                                                                                    end: 3026,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: EnumTag {
                                                                                    exp: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_4",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1049,
                                                                                                    end: 1050,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 1049,
                                                                                                end: 1050,
                                                                                                as_str(): "e",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31700,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 1049,
                                                                                            end: 1050,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1049,
                                                                                    end: 1050,
                                                                                    as_str(): "e",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3028,
                                                                                    end: 3033,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    U64(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1049,
                                                                                    end: 1050,
                                                                                    as_str(): "e",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13365,
                                                                        Span {
                                                                            src (ptr): 0x00007fe05c40b7f0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3016,
                                                                            end: 3082,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 1049,
                                                                    end: 1050,
                                                                    as_str(): "e",
                                                                },
                                                            },
                                                            rhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1061,
                                                                                    end: 1069,
                                                                                    as_str(): "E::X(51)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1061,
                                                                                    end: 1069,
                                                                                    as_str(): "E::X(51)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1061,
                                                                                end: 1069,
                                                                                as_str(): "E::X(51)",
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
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3022,
                                                                                    end: 3026,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: UnsafeDowncast {
                                                                                    exp: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_4",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1049,
                                                                                                    end: 1050,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 1049,
                                                                                                end: 1050,
                                                                                                as_str(): "e",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31700,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 1049,
                                                                                            end: 1050,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                    },
                                                                                    variant: TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 394,
                                                                                                end: 395,
                                                                                                as_str(): "X",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 397,
                                                                                            end: 400,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 394,
                                                                                            end: 400,
                                                                                            as_str(): "X: u64",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1061,
                                                                                    end: 1069,
                                                                                    as_str(): "E::X(51)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c40b7f0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3028,
                                                                                    end: 3033,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    Numeric(
                                                                                        51,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1066,
                                                                                    end: 1068,
                                                                                    as_str(): "51",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13364,
                                                                        Span {
                                                                            src (ptr): 0x00007fe05c40b7f0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3016,
                                                                            end: 3082,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 1061,
                                                                    end: 1069,
                                                                    as_str(): "E::X(51)",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1049,
                                                            end: 1069,
                                                            as_str(): "e {\n        E::X(51)",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31853,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1073,
                                                            end: 1075,
                                                            as_str(): "{}",
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
                                                                                    expression: FunctionApplication {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1090,
                                                                                                    end: 1096,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        contract_call_params: {},
                                                                                        arguments: [
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe050cc3b50,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                                        ),
                                                                                                        start: 582,
                                                                                                        end: 586,
                                                                                                        as_str(): "code",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: Literal(
                                                                                                        U64(
                                                                                                            0,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1097,
                                                                                                        end: 1098,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13363,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe050cc3b50,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                                ),
                                                                                                start: 568,
                                                                                                end: 615,
                                                                                                as_str(): "pub fn revert(code: u64) {\n    __revert(code)\n}",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: Some(
                                                                                            TypeBinding {
                                                                                                inner: (),
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1090,
                                                                                                    end: 1096,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        31859,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1090,
                                                                                        end: 1099,
                                                                                        as_str(): "revert(0)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1090,
                                                                                end: 1099,
                                                                                as_str(): "revert(0)",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                31859,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 1090,
                                                                end: 1099,
                                                                as_str(): "revert(0)",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    31860,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 1073,
                                                    end: 1075,
                                                    as_str(): "{}",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 1043,
                                            end: 1106,
                                            as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            31861,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 1043,
                            end: 1106,
                            as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 1043,
                    end: 1106,
                    as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
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
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 1116,
                            end: 1120,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe04ae0ff10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                    ),
                    start: 1116,
                    end: 1120,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe04ae0ff10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
        ),
        start: 592,
        end: 1122,
        as_str(): "fn main() -> bool {\n    let mut a = A {\n        a: 0,\n        b: 0,\n    };\n\n    a.f();\n    assert(a.a == 42);\n    assert(a.b == 77);\n\n    a.g(1);\n    assert(a.a == 43);\n    assert(a.b == 78);\n\n    a.h();\n    assert(a.a == 100);\n    assert(a.b == 200);\n\n    let mut e = E::X(42);\n    match e {\n        E::X(42) => {},\n        _ => revert(0),\n    };\n    \n    e.j(4);\n    match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    };\n\n    e.j(5);\n    match e {\n        E::X(51) => {},\n        _ => revert(0),\n    };\n   \n    true\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe04ae0ff10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
        ),
        start: 605,
        end: 609,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

