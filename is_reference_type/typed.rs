
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 42,
            end: 43,
            as_str(): "S",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 50,
                    end: 51,
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
                src (ptr): 0x00007fe0c08fd1d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                ),
                start: 50,
                end: 56,
                as_str(): "a: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0c08fd1d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                ),
                start: 53,
                end: 56,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0c08fd1d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
        ),
        start: 35,
        end: 59,
        as_str(): "struct S {\n    a: u64,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 66,
            end: 67,
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
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 74,
                    end: 81,
                    as_str(): "Variant",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31639,
            ),
            initial_type_id: TypeId(
                31638,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0c08fd1d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                ),
                start: 83,
                end: 85,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0c08fd1d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                ),
                start: 74,
                end: 85,
                as_str(): "Variant: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0c08fd1d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
        ),
        start: 61,
        end: 88,
        as_str(): "enum E {\n    Variant: (),\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 93,
            end: 109,
            as_str(): "arg_is_reference",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IntrinsicFunction(
                            TyIntrinsicFunctionKind {
                                kind: IsReferenceType,
                                arguments: [],
                                type_arguments: [
                                    TypeArgument {
                                        type_id: TypeId(
                                            31641,
                                        ),
                                        initial_type_id: TypeId(
                                            31643,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 155,
                                            end: 156,
                                            as_str(): "T",
                                        },
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 133,
                                    end: 159,
                                    as_str(): "__is_reference_type::<T>()",
                                },
                            },
                        ),
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 133,
                            end: 159,
                            as_str(): "__is_reference_type::<T>()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 133,
                    end: 159,
                    as_str(): "__is_reference_type::<T>()",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 113,
                    end: 114,
                    as_str(): "a",
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
            type_id: TypeId(
                31641,
            ),
            initial_type_id: TypeId(
                31642,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0c08fd1d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                ),
                start: 116,
                end: 117,
                as_str(): "T",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c08fd1d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
        ),
        start: 90,
        end: 161,
        as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
    type_parameters: [
        T: TypeId(31641),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0c08fd1d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
        ),
        start: 122,
        end: 126,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 166,
            end: 170,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 187,
                                        end: 193,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 195,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 195,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "not",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 194,
                                                        end: 195,
                                                        as_str(): "!",
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
                                                            src (ptr): 0x00007fe0cf48e840,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2717,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: IsReferenceType,
                                                                arguments: [],
                                                                type_arguments: [
                                                                    TypeArgument {
                                                                        type_id: TypeId(
                                                                            31649,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            31648,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 217,
                                                                            end: 219,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 195,
                                                                    end: 222,
                                                                    as_str(): "__is_reference_type::<()>()",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 195,
                                                            end: 222,
                                                            as_str(): "__is_reference_type::<()>()",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13318,
                                                Span {
                                                    src (ptr): 0x00007fe0cf48e840,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2706,
                                                    end: 2760,
                                                    as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 194,
                                                        end: 195,
                                                        as_str(): "!",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 194,
                                            end: 222,
                                            as_str(): "!__is_reference_type::<()>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13319,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 187,
                                        end: 193,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31650,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 187,
                            end: 223,
                            as_str(): "assert(!__is_reference_type::<()>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 187,
                    end: 223,
                    as_str(): "assert(!__is_reference_type::<()>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 259,
                                        end: 265,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 266,
                                                            end: 267,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 266,
                                                            end: 267,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "not",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 266,
                                                        end: 267,
                                                        as_str(): "!",
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
                                                            src (ptr): 0x00007fe0cf48e840,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2717,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: IsReferenceType,
                                                                arguments: [],
                                                                type_arguments: [
                                                                    TypeArgument {
                                                                        type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 289,
                                                                            end: 293,
                                                                            as_str(): "bool",
                                                                        },
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 267,
                                                                    end: 296,
                                                                    as_str(): "__is_reference_type::<bool>()",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 267,
                                                            end: 296,
                                                            as_str(): "__is_reference_type::<bool>()",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13321,
                                                Span {
                                                    src (ptr): 0x00007fe0cf48e840,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2706,
                                                    end: 2760,
                                                    as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 266,
                                                        end: 267,
                                                        as_str(): "!",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 266,
                                            end: 296,
                                            as_str(): "!__is_reference_type::<bool>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13322,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 259,
                                        end: 265,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31654,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 259,
                            end: 297,
                            as_str(): "assert(!__is_reference_type::<bool>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 259,
                    end: 297,
                    as_str(): "assert(!__is_reference_type::<bool>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 303,
                                        end: 309,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 310,
                                                            end: 311,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 310,
                                                            end: 311,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "not",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 310,
                                                        end: 311,
                                                        as_str(): "!",
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
                                                            src (ptr): 0x00007fe0cf48e840,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2717,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: IsReferenceType,
                                                                arguments: [],
                                                                type_arguments: [
                                                                    TypeArgument {
                                                                        type_id: TypeId(
                                                                            21,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 333,
                                                                            end: 336,
                                                                            as_str(): "u64",
                                                                        },
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 311,
                                                                    end: 339,
                                                                    as_str(): "__is_reference_type::<u64>()",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 311,
                                                            end: 339,
                                                            as_str(): "__is_reference_type::<u64>()",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13324,
                                                Span {
                                                    src (ptr): 0x00007fe0cf48e840,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2706,
                                                    end: 2760,
                                                    as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 310,
                                                        end: 311,
                                                        as_str(): "!",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 310,
                                            end: 339,
                                            as_str(): "!__is_reference_type::<u64>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 303,
                                        end: 309,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31658,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 303,
                            end: 340,
                            as_str(): "assert(!__is_reference_type::<u64>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 303,
                    end: 340,
                    as_str(): "assert(!__is_reference_type::<u64>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 353,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                        expression: IntrinsicFunction(
                                            TyIntrinsicFunctionKind {
                                                kind: IsReferenceType,
                                                arguments: [],
                                                type_arguments: [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            31631,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 376,
                                                            end: 382,
                                                            as_str(): "str[1]",
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 354,
                                                    end: 385,
                                                    as_str(): "__is_reference_type::<str[1]>()",
                                                },
                                            },
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 354,
                                            end: 385,
                                            as_str(): "__is_reference_type::<str[1]>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13327,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 353,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31661,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 347,
                            end: 386,
                            as_str(): "assert(__is_reference_type::<str[1]>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 347,
                    end: 386,
                    as_str(): "assert(__is_reference_type::<str[1]>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 392,
                                        end: 398,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                        expression: IntrinsicFunction(
                                            TyIntrinsicFunctionKind {
                                                kind: IsReferenceType,
                                                arguments: [],
                                                type_arguments: [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            59,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 421,
                                                            end: 425,
                                                            as_str(): "b256",
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 399,
                                                    end: 428,
                                                    as_str(): "__is_reference_type::<b256>()",
                                                },
                                            },
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 399,
                                            end: 428,
                                            as_str(): "__is_reference_type::<b256>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13329,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 392,
                                        end: 398,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31664,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 392,
                            end: 429,
                            as_str(): "assert(__is_reference_type::<b256>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 392,
                    end: 429,
                    as_str(): "assert(__is_reference_type::<b256>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 435,
                                        end: 441,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                        expression: IntrinsicFunction(
                                            TyIntrinsicFunctionKind {
                                                kind: IsReferenceType,
                                                arguments: [],
                                                type_arguments: [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            31668,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            31667,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 465,
                                                            as_str(): "S",
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 442,
                                                    end: 468,
                                                    as_str(): "__is_reference_type::<S>()",
                                                },
                                            },
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 442,
                                            end: 468,
                                            as_str(): "__is_reference_type::<S>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13331,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 435,
                                        end: 441,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31669,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 435,
                            end: 469,
                            as_str(): "assert(__is_reference_type::<S>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 435,
                    end: 469,
                    as_str(): "assert(__is_reference_type::<S>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 475,
                                        end: 481,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                        expression: IntrinsicFunction(
                                            TyIntrinsicFunctionKind {
                                                kind: IsReferenceType,
                                                arguments: [],
                                                type_arguments: [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            31673,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            31672,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 504,
                                                            end: 505,
                                                            as_str(): "E",
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 482,
                                                    end: 508,
                                                    as_str(): "__is_reference_type::<E>()",
                                                },
                                            },
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 482,
                                            end: 508,
                                            as_str(): "__is_reference_type::<E>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 475,
                                        end: 481,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31674,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 475,
                            end: 509,
                            as_str(): "assert(__is_reference_type::<E>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 475,
                    end: 509,
                    as_str(): "assert(__is_reference_type::<E>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 515,
                                        end: 521,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                        expression: IntrinsicFunction(
                                            TyIntrinsicFunctionKind {
                                                kind: IsReferenceType,
                                                arguments: [],
                                                type_arguments: [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            31678,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            31677,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 544,
                                                            end: 556,
                                                            as_str(): "(bool, bool)",
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 522,
                                                    end: 559,
                                                    as_str(): "__is_reference_type::<(bool, bool)>()",
                                                },
                                            },
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 522,
                                            end: 559,
                                            as_str(): "__is_reference_type::<(bool, bool)>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13335,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 515,
                                        end: 521,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31679,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 515,
                            end: 560,
                            as_str(): "assert(__is_reference_type::<(bool, bool)>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 515,
                    end: 560,
                    as_str(): "assert(__is_reference_type::<(bool, bool)>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 566,
                                        end: 572,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                        expression: IntrinsicFunction(
                                            TyIntrinsicFunctionKind {
                                                kind: IsReferenceType,
                                                arguments: [],
                                                type_arguments: [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            31683,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            31682,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 595,
                                                            end: 603,
                                                            as_str(): "[u64; 2]",
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 573,
                                                    end: 606,
                                                    as_str(): "__is_reference_type::<[u64; 2]>()",
                                                },
                                            },
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 573,
                                            end: 606,
                                            as_str(): "__is_reference_type::<[u64; 2]>()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 566,
                                        end: 572,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31684,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 566,
                            end: 607,
                            as_str(): "assert(__is_reference_type::<[u64; 2]>())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 566,
                    end: 607,
                    as_str(): "assert(__is_reference_type::<[u64; 2]>())",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 614,
                                        end: 620,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 621,
                                                            end: 622,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 621,
                                                            end: 622,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "not",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 621,
                                                        end: 622,
                                                        as_str(): "!",
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
                                                            src (ptr): 0x00007fe0cf48e840,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2717,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 622,
                                                                        end: 638,
                                                                        as_str(): "arg_is_reference",
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 114,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Tuple {
                                                                            fields: [],
                                                                        },
                                                                        return_type: TypeId(
                                                                            31691,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 639,
                                                                            end: 641,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13340,
                                                                Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 90,
                                                                    end: 161,
                                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 622,
                                                                        end: 638,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 622,
                                                            end: 642,
                                                            as_str(): "arg_is_reference(())",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13341,
                                                Span {
                                                    src (ptr): 0x00007fe0cf48e840,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2706,
                                                    end: 2760,
                                                    as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 621,
                                                        end: 622,
                                                        as_str(): "!",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 621,
                                            end: 642,
                                            as_str(): "!arg_is_reference(())",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13342,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 614,
                                        end: 620,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31692,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 614,
                            end: 643,
                            as_str(): "assert(!arg_is_reference(()))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 614,
                    end: 643,
                    as_str(): "assert(!arg_is_reference(()))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 649,
                                        end: 655,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 656,
                                                            end: 657,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 656,
                                                            end: 657,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "not",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 656,
                                                        end: 657,
                                                        as_str(): "!",
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
                                                            src (ptr): 0x00007fe0cf48e840,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2717,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 657,
                                                                        end: 673,
                                                                        as_str(): "arg_is_reference",
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 114,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 674,
                                                                            end: 679,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13345,
                                                                Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 90,
                                                                    end: 161,
                                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 657,
                                                                        end: 673,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 657,
                                                            end: 680,
                                                            as_str(): "arg_is_reference(false)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13346,
                                                Span {
                                                    src (ptr): 0x00007fe0cf48e840,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2706,
                                                    end: 2760,
                                                    as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 656,
                                                        end: 657,
                                                        as_str(): "!",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 656,
                                            end: 680,
                                            as_str(): "!arg_is_reference(false)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13347,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 649,
                                        end: 655,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31698,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 649,
                            end: 681,
                            as_str(): "assert(!arg_is_reference(false))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 649,
                    end: 681,
                    as_str(): "assert(!arg_is_reference(false))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 687,
                                        end: 693,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 694,
                                                            end: 695,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 694,
                                                            end: 695,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "not",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 694,
                                                        end: 695,
                                                        as_str(): "!",
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
                                                            src (ptr): 0x00007fe0cf48e840,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2717,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 695,
                                                                        end: 711,
                                                                        as_str(): "arg_is_reference",
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 114,
                                                                            as_str(): "a",
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 712,
                                                                            end: 716,
                                                                            as_str(): "0x2b",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13350,
                                                                Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 90,
                                                                    end: 161,
                                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 695,
                                                                        end: 711,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 695,
                                                            end: 717,
                                                            as_str(): "arg_is_reference(0x2b)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13351,
                                                Span {
                                                    src (ptr): 0x00007fe0cf48e840,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2706,
                                                    end: 2760,
                                                    as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 694,
                                                        end: 695,
                                                        as_str(): "!",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 694,
                                            end: 717,
                                            as_str(): "!arg_is_reference(0x2b)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13352,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 687,
                                        end: 693,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31704,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 687,
                            end: 718,
                            as_str(): "assert(!arg_is_reference(0x2b))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 687,
                    end: 718,
                    as_str(): "assert(!arg_is_reference(0x2b))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 724,
                                        end: 730,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 731,
                                                            end: 732,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 731,
                                                            end: 732,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "not",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 731,
                                                        end: 732,
                                                        as_str(): "!",
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
                                                            src (ptr): 0x00007fe0cf48e840,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2717,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 732,
                                                                        end: 748,
                                                                        as_str(): "arg_is_reference",
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 114,
                                                                            as_str(): "a",
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 749,
                                                                            end: 750,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13355,
                                                                Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 90,
                                                                    end: 161,
                                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 732,
                                                                        end: 748,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 732,
                                                            end: 751,
                                                            as_str(): "arg_is_reference(0)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13356,
                                                Span {
                                                    src (ptr): 0x00007fe0cf48e840,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2706,
                                                    end: 2760,
                                                    as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 731,
                                                        end: 732,
                                                        as_str(): "!",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 731,
                                            end: 751,
                                            as_str(): "!arg_is_reference(0)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13357,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 724,
                                        end: 730,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31711,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 724,
                            end: 752,
                            as_str(): "assert(!arg_is_reference(0))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 724,
                    end: 752,
                    as_str(): "assert(!arg_is_reference(0))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 759,
                                        end: 765,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 766,
                                                        end: 782,
                                                        as_str(): "arg_is_reference",
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            String(
                                                                Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 784,
                                                                    end: 793,
                                                                    as_str(): "breakfast",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            31716,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 783,
                                                            end: 794,
                                                            as_str(): "\"breakfast\"",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13360,
                                                Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 161,
                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 766,
                                                        end: 782,
                                                        as_str(): "arg_is_reference",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 766,
                                            end: 795,
                                            as_str(): "arg_is_reference(\"breakfast\")",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 759,
                                        end: 765,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31717,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 759,
                            end: 796,
                            as_str(): "assert(arg_is_reference(\"breakfast\"))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 759,
                    end: 796,
                    as_str(): "assert(arg_is_reference(\"breakfast\"))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 802,
                                        end: 808,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 809,
                                                        end: 825,
                                                        as_str(): "arg_is_reference",
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            B256(
                                                                [
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                    254,
                                                                ],
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 826,
                                                            end: 892,
                                                            as_str(): "0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13364,
                                                Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 161,
                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 809,
                                                        end: 825,
                                                        as_str(): "arg_is_reference",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 809,
                                            end: 893,
                                            as_str(): "arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13365,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 802,
                                        end: 808,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31722,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 802,
                            end: 894,
                            as_str(): "assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 802,
                    end: 894,
                    as_str(): "assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 900,
                                        end: 906,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 907,
                                                        end: 923,
                                                        as_str(): "arg_is_reference",
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructExpression {
                                                            struct_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 42,
                                                                    end: 43,
                                                                    as_str(): "S",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            fields: [
                                                                TyStructExpressionField {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 928,
                                                                            end: 929,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    value: TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 931,
                                                                            end: 933,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 924,
                                                                end: 925,
                                                                as_str(): "S",
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            31668,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 924,
                                                            end: 935,
                                                            as_str(): "S { a: 42 }",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13368,
                                                Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 161,
                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 907,
                                                        end: 923,
                                                        as_str(): "arg_is_reference",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 907,
                                            end: 936,
                                            as_str(): "arg_is_reference(S { a: 42 })",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13369,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 900,
                                        end: 906,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31730,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 900,
                            end: 937,
                            as_str(): "assert(arg_is_reference(S { a: 42 }))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 900,
                    end: 937,
                    as_str(): "assert(arg_is_reference(S { a: 42 }))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 943,
                                        end: 949,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 950,
                                                        end: 966,
                                                        as_str(): "arg_is_reference",
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 66,
                                                                        end: 67,
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
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 74,
                                                                                end: 81,
                                                                                as_str(): "Variant",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            31639,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            31638,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 83,
                                                                            end: 85,
                                                                            as_str(): "()",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 74,
                                                                            end: 85,
                                                                            as_str(): "Variant: ()",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 61,
                                                                    end: 88,
                                                                    as_str(): "enum E {\n    Variant: (),\n}",
                                                                },
                                                                visibility: Private,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 74,
                                                                    end: 81,
                                                                    as_str(): "Variant",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 0,
                                                            contents: None,
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 967,
                                                                end: 968,
                                                                as_str(): "E",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 970,
                                                                end: 977,
                                                                as_str(): "Variant",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 970,
                                                                    end: 977,
                                                                    as_str(): "Variant",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            31673,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 970,
                                                            end: 977,
                                                            as_str(): "Variant",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13373,
                                                Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 161,
                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 950,
                                                        end: 966,
                                                        as_str(): "arg_is_reference",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 950,
                                            end: 978,
                                            as_str(): "arg_is_reference(E::Variant)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13374,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 943,
                                        end: 949,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31735,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 943,
                            end: 979,
                            as_str(): "assert(arg_is_reference(E::Variant))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 943,
                    end: 979,
                    as_str(): "assert(arg_is_reference(E::Variant))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 985,
                                        end: 991,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 992,
                                                        end: 1008,
                                                        as_str(): "arg_is_reference",
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Tuple {
                                                            fields: [
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
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1010,
                                                                        end: 1014,
                                                                        as_str(): "true",
                                                                    },
                                                                },
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
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1016,
                                                                        end: 1020,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        return_type: TypeId(
                                                            31743,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 1009,
                                                            end: 1021,
                                                            as_str(): "(true, true)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13377,
                                                Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 161,
                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 992,
                                                        end: 1008,
                                                        as_str(): "arg_is_reference",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 992,
                                            end: 1022,
                                            as_str(): "arg_is_reference((true, true))",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13378,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 985,
                                        end: 991,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31744,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 985,
                            end: 1023,
                            as_str(): "assert(arg_is_reference((true, true)))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 985,
                    end: 1023,
                    as_str(): "assert(arg_is_reference((true, true)))",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 1029,
                                        end: 1035,
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
                                            src (ptr): 0x00007fe0c8cf7110,
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
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 1036,
                                                        end: 1052,
                                                        as_str(): "arg_is_reference",
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
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Array {
                                                            contents: [
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
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1054,
                                                                        end: 1055,
                                                                        as_str(): "5",
                                                                    },
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
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1057,
                                                                        end: 1058,
                                                                        as_str(): "4",
                                                                    },
                                                                },
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1060,
                                                                        end: 1061,
                                                                        as_str(): "3",
                                                                    },
                                                                },
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1063,
                                                                        end: 1064,
                                                                        as_str(): "2",
                                                                    },
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
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1066,
                                                                        end: 1067,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        return_type: TypeId(
                                                            31760,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 1053,
                                                            end: 1068,
                                                            as_str(): "[5, 4, 3, 2, 1]",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13381,
                                                Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 161,
                                                    as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 1036,
                                                        end: 1052,
                                                        as_str(): "arg_is_reference",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 1036,
                                            end: 1069,
                                            as_str(): "arg_is_reference([5, 4, 3, 2, 1])",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13382,
                                Span {
                                    src (ptr): 0x00007fe0c8cf7110,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 1029,
                                        end: 1035,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31761,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 1029,
                            end: 1070,
                            as_str(): "assert(arg_is_reference([5, 4, 3, 2, 1]))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 1029,
                    end: 1070,
                    as_str(): "assert(arg_is_reference([5, 4, 3, 2, 1]))",
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
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 1077,
                            end: 1081,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c08fd1d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                    ),
                    start: 1077,
                    end: 1081,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c08fd1d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
        ),
        start: 163,
        end: 1083,
        as_str(): "fn main() -> bool {\n    assert(!__is_reference_type::<()>());        // Is Unit ref or not?\n    assert(!__is_reference_type::<bool>());\n    assert(!__is_reference_type::<u64>());\n\n    assert(__is_reference_type::<str[1]>());\n    assert(__is_reference_type::<b256>());\n    assert(__is_reference_type::<S>());\n    assert(__is_reference_type::<E>());\n    assert(__is_reference_type::<(bool, bool)>());\n    assert(__is_reference_type::<[u64; 2]>());\n\n    assert(!arg_is_reference(()));\n    assert(!arg_is_reference(false));\n    assert(!arg_is_reference(0x2b));\n    assert(!arg_is_reference(0));\n\n    assert(arg_is_reference(\"breakfast\"));\n    assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe));\n    assert(arg_is_reference(S { a: 42 }));\n    assert(arg_is_reference(E::Variant));\n    assert(arg_is_reference((true, true)));\n    assert(arg_is_reference([5, 4, 3, 2, 1]));\n\n    true\n}",
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
        src (ptr): 0x00007fe0c08fd1d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
        ),
        start: 176,
        end: 180,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

