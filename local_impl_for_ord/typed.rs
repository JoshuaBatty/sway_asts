

TyEnumDeclaration {
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
    type_parameters: [],
    attributes: {},
    variants: [
        TyEnumVariant {
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
            type_id: TypeId(
                7252,
            ),
            initial_type_id: TypeId(
                7251,
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
            attributes: {},
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
}
ImplTrait(
    DeclId(
        551,
        Span {
            src (ptr): 0x00007fe0bc9cf310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
            ),
            start: 59,
            end: 134,
            as_str(): "impl Eq for X {\n    fn eq(self, other: Self) -> bool {\n        true\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        554,
        Span {
            src (ptr): 0x00007fe0bc9cf310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
            ),
            start: 136,
            end: 272,
            as_str(): "impl Ord for X {\n    fn lt(self, other: Self) -> bool {\n        false\n    }\n    fn gt(self, other: Self) -> bool {\n        false\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
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
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: EnumInstantiation {
                                            enum_decl: TyEnumDeclaration {
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
                                                type_parameters: [],
                                                attributes: {},
                                                variants: [
                                                    TyEnumVariant {
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
                                                        type_id: TypeId(
                                                            7252,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7251,
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
                                                        attributes: {},
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
                                            variant_name: BaseIdent {
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
                                            tag: 0,
                                            contents: None,
                                            enum_instantiation_span: Span {
                                                src (ptr): 0x00007fe0bc9cf310,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                ),
                                                start: 298,
                                                end: 299,
                                                as_str(): "X",
                                            },
                                            variant_instantiation_span: Span {
                                                src (ptr): 0x00007fe0bc9cf310,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                ),
                                                start: 301,
                                                end: 302,
                                                as_str(): "Y",
                                            },
                                            type_binding: TypeBinding {
                                                inner: (),
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
                                        },
                                        return_type: TypeId(
                                            7254,
                                        ),
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
                                ),
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: EnumInstantiation {
                                            enum_decl: TyEnumDeclaration {
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
                                                type_parameters: [],
                                                attributes: {},
                                                variants: [
                                                    TyEnumVariant {
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
                                                        type_id: TypeId(
                                                            7252,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7251,
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
                                                        attributes: {},
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
                                            variant_name: BaseIdent {
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
                                            tag: 0,
                                            contents: None,
                                            enum_instantiation_span: Span {
                                                src (ptr): 0x00007fe0bc9cf310,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                ),
                                                start: 306,
                                                end: 307,
                                                as_str(): "X",
                                            },
                                            variant_instantiation_span: Span {
                                                src (ptr): 0x00007fe0bc9cf310,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                ),
                                                start: 309,
                                                end: 310,
                                                as_str(): "Y",
                                            },
                                            type_binding: TypeBinding {
                                                inner: (),
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
                                        },
                                        return_type: TypeId(
                                            7254,
                                        ),
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
                                ),
                            ],
                            function_decl_id: DeclId(
                                557,
                                Span {
                                    src (ptr): 0x00007fe0bc9cf310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                    ),
                                    start: 79,
                                    end: 132,
                                    as_str(): "fn eq(self, other: Self) -> bool {\n        true\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            71,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0bc9cf310,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
        ),
        start: 274,
        end: 312,
        as_str(): "fn main() -> bool {\n    X::Y == X::Y\n}",
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
        src (ptr): 0x00007fe0bc9cf310,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
        ),
        start: 287,
        end: 291,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

