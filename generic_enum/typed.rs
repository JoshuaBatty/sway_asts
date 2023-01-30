
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc4e9d60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
            ),
            start: 158,
            end: 164,
            as_str(): "Option",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7252),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc4e9d60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                    ),
                    start: 174,
                    end: 178,
                    as_str(): "Some",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7252,
            ),
            initial_type_id: TypeId(
                7253,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc4e9d60,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                ),
                start: 180,
                end: 181,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0fc4e9d60,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                ),
                start: 174,
                end: 181,
                as_str(): "Some: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc4e9d60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                    ),
                    start: 187,
                    end: 191,
                    as_str(): "None",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7255,
            ),
            initial_type_id: TypeId(
                7254,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc4e9d60,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                ),
                start: 193,
                end: 195,
                as_str(): "()",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0fc4e9d60,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                ),
                start: 187,
                end: 195,
                as_str(): "None: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0fc4e9d60,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
        ),
        start: 153,
        end: 197,
        as_str(): "enum Option<T> {\n    Some: T,\n    None: ()\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc4e9d60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
            ),
            start: 32,
            end: 36,
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
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 57,
                                    end: 58,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fc4e9d60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                ),
                                                start: 158,
                                                end: 164,
                                                as_str(): "Option",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(7258),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc4e9d60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                        ),
                                                        start: 174,
                                                        end: 178,
                                                        as_str(): "Some",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7258,
                                                ),
                                                initial_type_id: TypeId(
                                                    7253,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 180,
                                                    end: 181,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 181,
                                                    as_str(): "Some: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc4e9d60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                        ),
                                                        start: 187,
                                                        end: 191,
                                                        as_str(): "None",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7255,
                                                ),
                                                initial_type_id: TypeId(
                                                    7254,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 193,
                                                    end: 195,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 187,
                                                    end: 195,
                                                    as_str(): "None: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 153,
                                            end: 197,
                                            as_str(): "enum Option<T> {\n    Some: T,\n    None: ()\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 174,
                                            end: 178,
                                            as_str(): "Some",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    10,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fc4e9d60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                ),
                                                start: 74,
                                                end: 76,
                                                as_str(): "10",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 67,
                                        as_str(): "Option",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 73,
                                        as_str(): "Some",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 61,
                                            end: 73,
                                            as_str(): "Option::Some",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7262,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 69,
                                    end: 73,
                                    as_str(): "Some",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7262,
                            ),
                            type_ascription: TypeId(
                                7257,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc4e9d60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                    ),
                    start: 53,
                    end: 78,
                    as_str(): "let x = Option::Some(10);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 88,
                                    end: 89,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fc4e9d60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                ),
                                                start: 158,
                                                end: 164,
                                                as_str(): "Option",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(7264),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc4e9d60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                        ),
                                                        start: 174,
                                                        end: 178,
                                                        as_str(): "Some",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7264,
                                                ),
                                                initial_type_id: TypeId(
                                                    7253,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 180,
                                                    end: 181,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 181,
                                                    as_str(): "Some: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc4e9d60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                        ),
                                                        start: 187,
                                                        end: 191,
                                                        as_str(): "None",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7255,
                                                ),
                                                initial_type_id: TypeId(
                                                    7254,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 193,
                                                    end: 195,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 187,
                                                    end: 195,
                                                    as_str(): "None: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 153,
                                            end: 197,
                                            as_str(): "enum Option<T> {\n    Some: T,\n    None: ()\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 174,
                                            end: 178,
                                            as_str(): "Some",
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
                                                src (ptr): 0x00007fe0fc4e9d60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                ),
                                                start: 105,
                                                end: 109,
                                                as_str(): "true",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 98,
                                        as_str(): "Option",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 100,
                                        end: 104,
                                        as_str(): "Some",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 92,
                                            end: 104,
                                            as_str(): "Option::Some",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7267,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 100,
                                    end: 104,
                                    as_str(): "Some",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7267,
                            ),
                            type_ascription: TypeId(
                                7263,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc4e9d60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                    ),
                    start: 84,
                    end: 111,
                    as_str(): "let y = Option::Some(true);",
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
                            src (ptr): 0x00007fe0fc4e9d60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                            ),
                            start: 145,
                            end: 149,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc4e9d60,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                    ),
                    start: 145,
                    end: 149,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc4e9d60,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
        ),
        start: 29,
        end: 151,
        as_str(): "fn main() -> bool {\n    let x = Option::Some(10); \n    let y = Option::Some(true); \n\n //   x == Option::Some(10)\n   true\n}",
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
        src (ptr): 0x00007fe0fc4e9d60,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
        ),
        start: 42,
        end: 46,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

