

TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 53,
            end: 60,
            as_str(): "Result2",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(31641),
        E: TypeId(31642),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 73,
                    end: 75,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31641,
            ),
            initial_type_id: TypeId(
                31643,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f97a4fa0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                ),
                start: 77,
                end: 78,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0f97a4fa0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                ),
                start: 73,
                end: 78,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 84,
                    end: 87,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31642,
            ),
            initial_type_id: TypeId(
                31644,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f97a4fa0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                ),
                start: 89,
                end: 90,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0f97a4fa0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                ),
                start: 84,
                end: 90,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0f97a4fa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
        ),
        start: 48,
        end: 93,
        as_str(): "enum Result2<T, E> {\n    Ok: T,\n    Err: E,\n}",
    },
    visibility: Private,
}
ImplTrait(
    DeclId(
        13318,
        Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 95,
            end: 299,
            as_str(): "impl<T, E> Result2<T, E> {\n    pub fn unwrap_or(self, default: T) -> T {\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 308,
            end: 322,
            as_str(): "test_unwrap_or",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 368,
                                        end: 374,
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
                                            src (ptr): 0x00007fe0fd2c2540,
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
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 419,
                                                            end: 421,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 419,
                                                            end: 421,
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
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 419,
                                                        end: 421,
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
                                                            src (ptr): 0x00007fe0fcd9c650,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2789,
                                                            end: 2793,
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
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 400,
                                                                        end: 409,
                                                                        as_str(): "unwrap_or",
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
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 143,
                                                                            end: 147,
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
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 53,
                                                                                        end: 60,
                                                                                        as_str(): "Result2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [
                                                                                    T: TypeId(31752),
                                                                                    E: TypeId(31752),
                                                                                ],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 73,
                                                                                                end: 75,
                                                                                                as_str(): "Ok",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31752,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31643,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 77,
                                                                                            end: 78,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 78,
                                                                                            as_str(): "Ok: T",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 84,
                                                                                                end: 87,
                                                                                                as_str(): "Err",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31752,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31644,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 89,
                                                                                            end: 90,
                                                                                            as_str(): "E",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 84,
                                                                                            end: 90,
                                                                                            as_str(): "Err: E",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 48,
                                                                                    end: 93,
                                                                                    as_str(): "enum Result2<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 73,
                                                                                    end: 75,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 0,
                                                                            contents: Some(
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 326,
                                                                                                end: 329,
                                                                                                as_str(): "val",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 395,
                                                                                            end: 398,
                                                                                            as_str(): "val",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        31752,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 395,
                                                                                        end: 398,
                                                                                        as_str(): "val",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 375,
                                                                                end: 382,
                                                                                as_str(): "Result2",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 384,
                                                                                end: 386,
                                                                                as_str(): "Ok",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [
                                                                                    TypeArgument {
                                                                                        type_id: TypeId(
                                                                                            31752,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31628,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 389,
                                                                                            end: 390,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                    },
                                                                                    TypeArgument {
                                                                                        type_id: TypeId(
                                                                                            31752,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31629,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 392,
                                                                                            end: 393,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 375,
                                                                                    end: 394,
                                                                                    as_str(): "Result2::Ok::<T, T>",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31859,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 384,
                                                                            end: 386,
                                                                            as_str(): "Ok",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 149,
                                                                            end: 156,
                                                                            as_str(): "default",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 334,
                                                                                    end: 341,
                                                                                    as_str(): "default",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 410,
                                                                                end: 417,
                                                                                as_str(): "default",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31752,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 410,
                                                                            end: 417,
                                                                            as_str(): "default",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13342,
                                                                Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 297,
                                                                    as_str(): "pub fn unwrap_or(self, default: T) -> T {\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 400,
                                                                        end: 409,
                                                                        as_str(): "unwrap_or",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31752,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 375,
                                                            end: 418,
                                                            as_str(): "Result2::Ok::<T, T>(val).unwrap_or(default)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fcd9c650,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2795,
                                                            end: 2800,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 326,
                                                                    end: 329,
                                                                    as_str(): "val",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 422,
                                                                end: 425,
                                                                as_str(): "val",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31752,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 422,
                                                            end: 425,
                                                            as_str(): "val",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13349,
                                                Span {
                                                    src (ptr): 0x00007fe0fcd9c650,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2786,
                                                    end: 2788,
                                                    as_str(): "eq",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 419,
                                                        end: 421,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 375,
                                            end: 425,
                                            as_str(): "Result2::Ok::<T, T>(val).unwrap_or(default) == val",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13362,
                                Span {
                                    src (ptr): 0x00007fe0fd2c2540,
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 368,
                                        end: 374,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31865,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 368,
                            end: 426,
                            as_str(): "assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 368,
                    end: 426,
                    as_str(): "assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val)",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 432,
                                        end: 438,
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
                                            src (ptr): 0x00007fe0fd2c2540,
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
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 484,
                                                            end: 486,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 484,
                                                            end: 486,
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
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 484,
                                                        end: 486,
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
                                                            src (ptr): 0x00007fe0fcd9c650,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2789,
                                                            end: 2793,
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
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 465,
                                                                        end: 474,
                                                                        as_str(): "unwrap_or",
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
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 143,
                                                                            end: 147,
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
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 53,
                                                                                        end: 60,
                                                                                        as_str(): "Result2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_parameters: [
                                                                                    T: TypeId(31752),
                                                                                    E: TypeId(31752),
                                                                                ],
                                                                                attributes: {},
                                                                                variants: [
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 73,
                                                                                                end: 75,
                                                                                                as_str(): "Ok",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31752,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31643,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 77,
                                                                                            end: 78,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                        tag: 0,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 78,
                                                                                            as_str(): "Ok: T",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    TyEnumVariant {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 84,
                                                                                                end: 87,
                                                                                                as_str(): "Err",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            31752,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31644,
                                                                                        ),
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 89,
                                                                                            end: 90,
                                                                                            as_str(): "E",
                                                                                        },
                                                                                        tag: 1,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 84,
                                                                                            end: 90,
                                                                                            as_str(): "Err: E",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 48,
                                                                                    end: 93,
                                                                                    as_str(): "enum Result2<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                                                                },
                                                                                visibility: Private,
                                                                            },
                                                                            variant_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 84,
                                                                                    end: 87,
                                                                                    as_str(): "Err",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            tag: 1,
                                                                            contents: Some(
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 326,
                                                                                                end: 329,
                                                                                                as_str(): "val",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 460,
                                                                                            end: 463,
                                                                                            as_str(): "val",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        31752,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 460,
                                                                                        end: 463,
                                                                                        as_str(): "val",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            enum_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 439,
                                                                                end: 446,
                                                                                as_str(): "Result2",
                                                                            },
                                                                            variant_instantiation_span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 448,
                                                                                end: 451,
                                                                                as_str(): "Err",
                                                                            },
                                                                            type_binding: TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [
                                                                                    TypeArgument {
                                                                                        type_id: TypeId(
                                                                                            31752,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31628,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 454,
                                                                                            end: 455,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                    },
                                                                                    TypeArgument {
                                                                                        type_id: TypeId(
                                                                                            31752,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            31629,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 457,
                                                                                            end: 458,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 439,
                                                                                    end: 459,
                                                                                    as_str(): "Result2::Err::<T, T>",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31899,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 448,
                                                                            end: 451,
                                                                            as_str(): "Err",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 149,
                                                                            end: 156,
                                                                            as_str(): "default",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 334,
                                                                                    end: 341,
                                                                                    as_str(): "default",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 475,
                                                                                end: 482,
                                                                                as_str(): "default",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31752,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 475,
                                                                            end: 482,
                                                                            as_str(): "default",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13393,
                                                                Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 297,
                                                                    as_str(): "pub fn unwrap_or(self, default: T) -> T {\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 465,
                                                                        end: 474,
                                                                        as_str(): "unwrap_or",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31752,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 439,
                                                            end: 483,
                                                            as_str(): "Result2::Err::<T, T>(val).unwrap_or(default)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fcd9c650,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2795,
                                                            end: 2800,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 334,
                                                                    end: 341,
                                                                    as_str(): "default",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 487,
                                                                end: 494,
                                                                as_str(): "default",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31752,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 487,
                                                            end: 494,
                                                            as_str(): "default",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13400,
                                                Span {
                                                    src (ptr): 0x00007fe0fcd9c650,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2786,
                                                    end: 2788,
                                                    as_str(): "eq",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 484,
                                                        end: 486,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 439,
                                            end: 494,
                                            as_str(): "Result2::Err::<T, T>(val).unwrap_or(default) == default",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13417,
                                Span {
                                    src (ptr): 0x00007fe0fd2c2540,
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 432,
                                        end: 438,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31906,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 432,
                            end: 495,
                            as_str(): "assert(Result2::Err::<T, T>(val).unwrap_or(default) == default)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 432,
                    end: 495,
                    as_str(): "assert(Result2::Err::<T, T>(val).unwrap_or(default) == default)",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 326,
                    end: 329,
                    as_str(): "val",
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
                31752,
            ),
            initial_type_id: TypeId(
                31839,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f97a4fa0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                ),
                start: 331,
                end: 332,
                as_str(): "T",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 334,
                    end: 341,
                    as_str(): "default",
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
                31752,
            ),
            initial_type_id: TypeId(
                31840,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f97a4fa0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                ),
                start: 343,
                end: 344,
                as_str(): "T",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f97a4fa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
        ),
        start: 301,
        end: 498,
        as_str(): "pub fn test_unwrap_or<T>(val: T, default: T)\nwhere\n    T: Eq\n{\n    assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val);\n    assert(Result2::Err::<T, T>(val).unwrap_or(default) == default);\n}",
    },
    attributes: {},
    return_type: TypeId(
        31842,
    ),
    initial_return_type: TypeId(
        31841,
    ),
    type_parameters: [
        T: TypeId(31752),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0f97a4fa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
        ),
        start: 301,
        end: 361,
        as_str(): "pub fn test_unwrap_or<T>(val: T, default: T)\nwhere\n    T: Eq",
    },
    visibility: Public,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 503,
            end: 507,
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 522,
                                        end: 536,
                                        as_str(): "test_unwrap_or",
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 326,
                                            end: 329,
                                            as_str(): "val",
                                        },
                                        is_raw_ident: false,
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 537,
                                            end: 541,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 334,
                                            end: 341,
                                            as_str(): "default",
                                        },
                                        is_raw_ident: false,
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 543,
                                            end: 547,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13450,
                                Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 301,
                                    end: 498,
                                    as_str(): "pub fn test_unwrap_or<T>(val: T, default: T)\nwhere\n    T: Eq\n{\n    assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val);\n    assert(Result2::Err::<T, T>(val).unwrap_or(default) == default);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 522,
                                        end: 536,
                                        as_str(): "test_unwrap_or",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32015,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 522,
                            end: 548,
                            as_str(): "test_unwrap_or(true, true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 522,
                    end: 548,
                    as_str(): "test_unwrap_or(true, true)",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 552,
                                        end: 566,
                                        as_str(): "test_unwrap_or",
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 326,
                                            end: 329,
                                            as_str(): "val",
                                        },
                                        is_raw_ident: false,
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 567,
                                            end: 571,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 334,
                                            end: 341,
                                            as_str(): "default",
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 573,
                                            end: 578,
                                            as_str(): "false",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13482,
                                Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 301,
                                    end: 498,
                                    as_str(): "pub fn test_unwrap_or<T>(val: T, default: T)\nwhere\n    T: Eq\n{\n    assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val);\n    assert(Result2::Err::<T, T>(val).unwrap_or(default) == default);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 552,
                                        end: 566,
                                        as_str(): "test_unwrap_or",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32122,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 552,
                            end: 579,
                            as_str(): "test_unwrap_or(true, false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 552,
                    end: 579,
                    as_str(): "test_unwrap_or(true, false)",
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
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 585,
                            end: 589,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f97a4fa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                    ),
                    start: 585,
                    end: 589,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f97a4fa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
        ),
        start: 500,
        end: 591,
        as_str(): "fn main() -> bool {\n  test_unwrap_or(true, true);\n  test_unwrap_or(true, false);\n\n\n  true\n}",
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
        src (ptr): 0x00007fe0f97a4fa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
        ),
        start: 513,
        end: 517,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

