TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 41,
            end: 45,
            as_str(): "Foo1",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 52,
                    end: 53,
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
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 52,
                end: 58,
                as_str(): "a: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 55,
                end: 58,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 64,
                    end: 65,
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
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 64,
                end: 70,
                as_str(): "b: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 67,
                end: 70,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 34,
        end: 73,
        as_str(): "struct Foo1 {\n    a: u64,\n    b: u64,\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        546,
        Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 75,
            end: 137,
            as_str(): "impl Foo1 {\n    fn trivial(self) -> bool {\n      false\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 142,
            end: 147,
            as_str(): "func1",
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
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 168,
                                    end: 169,
                                    as_str(): "f",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 41,
                                            end: 45,
                                            as_str(): "Foo1",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 178,
                                                    end: 179,
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 181,
                                                    end: 182,
                                                    as_str(): "0",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 184,
                                                    end: 185,
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 187,
                                                    end: 188,
                                                    as_str(): "0",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 172,
                                        end: 176,
                                        as_str(): "Foo1",
                                    },
                                },
                                return_type: TypeId(
                                    7265,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 172,
                                    end: 189,
                                    as_str(): "Foo1 {a: 0, b: 0}",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7265,
                            ),
                            type_ascription: TypeId(
                                7280,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 164,
                    end: 190,
                    as_str(): "let f = Foo1 {a: 0, b: 0};",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 197,
                                        end: 204,
                                        as_str(): "trivial",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 102,
                                            end: 106,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 168,
                                                    end: 169,
                                                    as_str(): "f",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 195,
                                                end: 196,
                                                as_str(): "f",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7265,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 195,
                                            end: 196,
                                            as_str(): "f",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                547,
                                Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 91,
                                    end: 135,
                                    as_str(): "fn trivial(self) -> bool {\n      false\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 197,
                                        end: 204,
                                        as_str(): "trivial",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 195,
                            end: 206,
                            as_str(): "f.trivial()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 195,
                    end: 206,
                    as_str(): "f.trivial()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 139,
        end: 208,
        as_str(): "fn func1() -> bool {\n    let f = Foo1 {a: 0, b: 0};\n    f.trivial()\n}",
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
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 153,
        end: 157,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 240,
            end: 243,
            as_str(): "Bar",
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
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 250,
                    end: 251,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7288,
            ),
            initial_type_id: TypeId(
                7287,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 253,
                end: 255,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 250,
                end: 255,
                as_str(): "a: ()",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 261,
                    end: 262,
                    as_str(): "b",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7290,
            ),
            initial_type_id: TypeId(
                7289,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 264,
                end: 266,
                as_str(): "()",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 261,
                end: 266,
                as_str(): "b: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 235,
        end: 269,
        as_str(): "enum Bar {\n    a: (),\n    b: (),\n}",
    },
    visibility: Private,
}
ImplTrait(
    DeclId(
        551,
        Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 271,
            end: 334,
            as_str(): "impl Bar {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 339,
            end: 344,
            as_str(): "func2",
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
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 369,
                                        end: 376,
                                        as_str(): "trivial",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 297,
                                            end: 301,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 345,
                                                    end: 346,
                                                    as_str(): "m",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 367,
                                                end: 368,
                                                as_str(): "m",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7292,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 367,
                                            end: 368,
                                            as_str(): "m",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                552,
                                Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 286,
                                    end: 332,
                                    as_str(): "fn trivial(self) -> bool {\n        false\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 369,
                                        end: 376,
                                        as_str(): "trivial",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 367,
                            end: 378,
                            as_str(): "m.trivial()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 367,
                    end: 378,
                    as_str(): "m.trivial()",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 345,
                    end: 346,
                    as_str(): "m",
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
                7292,
            ),
            initial_type_id: TypeId(
                7308,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 348,
                end: 351,
                as_str(): "Bar",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 336,
        end: 380,
        as_str(): "fn func2(m: Bar) -> bool {\n    m.trivial()\n}",
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
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 356,
        end: 360,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 414,
            end: 418,
            as_str(): "Foo2",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 428,
                    end: 431,
                    as_str(): "foo",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7310,
            ),
            initial_type_id: TypeId(
                7311,
            ),
            span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 428,
                end: 434,
                as_str(): "foo: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 433,
                end: 434,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7310),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 407,
        end: 437,
        as_str(): "struct Foo2<T> {\n    foo: T,\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        556,
        Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 439,
            end: 509,
            as_str(): "impl<T> Foo2<T> {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 514,
            end: 519,
            as_str(): "func3",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 558,
                                                end: 565,
                                                as_str(): "trivial",
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 472,
                                                    end: 476,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 520,
                                                            end: 521,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 556,
                                                        end: 557,
                                                        as_str(): "a",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7332,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 556,
                                                    end: 557,
                                                    as_str(): "a",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        559,
                                        Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 461,
                                            end: 507,
                                            as_str(): "fn trivial(self) -> bool {\n        false\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 558,
                                                end: 565,
                                                as_str(): "trivial",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 556,
                                    end: 567,
                                    as_str(): "a.trivial()",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: StructExpression {
                                                            struct_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 414,
                                                                    end: 418,
                                                                    as_str(): "Foo2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            fields: [
                                                                TyStructExpressionField {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 584,
                                                                            end: 587,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    value: TyExpression {
                                                                        expression: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 589,
                                                                            end: 594,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 578,
                                                                end: 582,
                                                                as_str(): "Foo2",
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            7343,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 578,
                                                            end: 595,
                                                            as_str(): "Foo2 {foo: false}",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 578,
                                                    end: 595,
                                                    as_str(): "Foo2 {foo: false}",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7343,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 568,
                                    end: 601,
                                    as_str(): "{\n        Foo2 {foo: false}\n    }",
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
                                                            expression: StructExpression {
                                                                struct_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 414,
                                                                        end: 418,
                                                                        as_str(): "Foo2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                fields: [
                                                                    TyStructExpressionField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 623,
                                                                                end: 626,
                                                                                as_str(): "foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value: TyExpression {
                                                                            expression: Literal(
                                                                                Boolean(
                                                                                    true,
                                                                                ),
                                                                            ),
                                                                            return_type: TypeId(
                                                                                71,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 628,
                                                                                end: 632,
                                                                                as_str(): "true",
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 617,
                                                                    end: 621,
                                                                    as_str(): "Foo2",
                                                                },
                                                            },
                                                            return_type: TypeId(
                                                                7350,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 617,
                                                                end: 633,
                                                                as_str(): "Foo2 {foo: true}",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 617,
                                                        end: 633,
                                                        as_str(): "Foo2 {foo: true}",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7350,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 607,
                                        end: 639,
                                        as_str(): "{\n        Foo2 {foo: true}\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7343,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 553,
                            end: 639,
                            as_str(): "if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 553,
                    end: 639,
                    as_str(): "if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 520,
                    end: 521,
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
                7332,
            ),
            initial_type_id: TypeId(
                7331,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 523,
                end: 531,
                as_str(): "Foo2<u8>",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 511,
        end: 641,
        as_str(): "fn func3(a: Foo2<u8>) -> Foo2<bool> {\n    if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        7336,
    ),
    initial_return_type: TypeId(
        7335,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 536,
        end: 546,
        as_str(): "Foo2<bool>",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 646,
            end: 651,
            as_str(): "func4",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 690,
                                                end: 697,
                                                as_str(): "trivial",
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 472,
                                                    end: 476,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 653,
                                                            as_str(): "b",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 688,
                                                        end: 689,
                                                        as_str(): "b",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7356,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 688,
                                                    end: 689,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        565,
                                        Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 461,
                                            end: 507,
                                            as_str(): "fn trivial(self) -> bool {\n        false\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 690,
                                                end: 697,
                                                as_str(): "trivial",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 688,
                                    end: 699,
                                    as_str(): "b.trivial()",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: StructExpression {
                                                            struct_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 414,
                                                                    end: 418,
                                                                    as_str(): "Foo2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            fields: [
                                                                TyStructExpressionField {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 716,
                                                                            end: 719,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    value: TyExpression {
                                                                        expression: Literal(
                                                                            U8(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 721,
                                                                            end: 724,
                                                                            as_str(): "0u8",
                                                                        },
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 710,
                                                                end: 714,
                                                                as_str(): "Foo2",
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            7367,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 710,
                                                            end: 725,
                                                            as_str(): "Foo2 {foo: 0u8}",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 710,
                                                    end: 725,
                                                    as_str(): "Foo2 {foo: 0u8}",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7367,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 700,
                                    end: 732,
                                    as_str(): "{\n        Foo2 {foo: 0u8} \n    }",
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
                                                            expression: StructExpression {
                                                                struct_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 414,
                                                                        end: 418,
                                                                        as_str(): "Foo2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                fields: [
                                                                    TyStructExpressionField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 754,
                                                                                end: 757,
                                                                                as_str(): "foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value: TyExpression {
                                                                            expression: Literal(
                                                                                U8(
                                                                                    1,
                                                                                ),
                                                                            ),
                                                                            return_type: TypeId(
                                                                                50,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 759,
                                                                                end: 762,
                                                                                as_str(): "1u8",
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 748,
                                                                    end: 752,
                                                                    as_str(): "Foo2",
                                                                },
                                                            },
                                                            return_type: TypeId(
                                                                7374,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 748,
                                                                end: 763,
                                                                as_str(): "Foo2 {foo: 1u8}",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 748,
                                                        end: 763,
                                                        as_str(): "Foo2 {foo: 1u8}",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7374,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 738,
                                        end: 769,
                                        as_str(): "{\n        Foo2 {foo: 1u8}\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7367,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 685,
                            end: 769,
                            as_str(): "if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 685,
                    end: 769,
                    as_str(): "if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 652,
                    end: 653,
                    as_str(): "b",
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
                7356,
            ),
            initial_type_id: TypeId(
                7355,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 655,
                end: 665,
                as_str(): "Foo2<bool>",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 643,
        end: 771,
        as_str(): "fn func4(b: Foo2<bool>) -> Foo2<u8> {\n    if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        7360,
    ),
    initial_return_type: TypeId(
        7359,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 670,
        end: 678,
        as_str(): "Foo2<u8>",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 807,
            end: 813,
            as_str(): "Rezult",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7378),
        E: TypeId(7379),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 826,
                    end: 828,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7378,
            ),
            initial_type_id: TypeId(
                7380,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 830,
                end: 831,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 826,
                end: 831,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 837,
                    end: 840,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7379,
            ),
            initial_type_id: TypeId(
                7381,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 842,
                end: 843,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 837,
                end: 843,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 798,
        end: 846,
        as_str(): "pub enum Rezult<T, E> {\n    Ok: T,\n    Err: E,\n}",
    },
    visibility: Public,
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 857,
            end: 866,
            as_str(): "DumbError",
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
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 873,
                    end: 878,
                    as_str(): "Error",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7383,
            ),
            initial_type_id: TypeId(
                7382,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 880,
                end: 882,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 873,
                end: 882,
                as_str(): "Error: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 848,
        end: 885,
        as_str(): "pub enum DumbError {\n    Error: (),\n}",
    },
    visibility: Public,
}
ImplTrait(
    DeclId(
        572,
        Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 887,
            end: 965,
            as_str(): "impl<T, E> Rezult<T, E> {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 974,
            end: 979,
            as_str(): "func5",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 1042,
                                                end: 1049,
                                                as_str(): "trivial",
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 928,
                                                    end: 932,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 980,
                                                            end: 981,
                                                            as_str(): "r",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 1040,
                                                        end: 1041,
                                                        as_str(): "r",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7407,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 1040,
                                                    end: 1041,
                                                    as_str(): "r",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        576,
                                        Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 917,
                                            end: 963,
                                            as_str(): "fn trivial(self) -> bool {\n        false\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 1042,
                                                end: 1049,
                                                as_str(): "trivial",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 1040,
                                    end: 1051,
                                    as_str(): "r.trivial()",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 807,
                                                                        end: 813,
                                                                        as_str(): "Rezult",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_parameters: [
                                                                    T: TypeId(7417),
                                                                    E: TypeId(7418),
                                                                ],
                                                                attributes: {},
                                                                variants: [
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 826,
                                                                                end: 828,
                                                                                as_str(): "Ok",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            7417,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            7380,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 830,
                                                                            end: 831,
                                                                            as_str(): "T",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 826,
                                                                            end: 831,
                                                                            as_str(): "Ok: T",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 837,
                                                                                end: 840,
                                                                                as_str(): "Err",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            7418,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            7381,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 842,
                                                                            end: 843,
                                                                            as_str(): "E",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 837,
                                                                            end: 843,
                                                                            as_str(): "Err: E",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 798,
                                                                    end: 846,
                                                                    as_str(): "pub enum Rezult<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                                                },
                                                                visibility: Public,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 837,
                                                                    end: 840,
                                                                    as_str(): "Err",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 1,
                                                            contents: Some(
                                                                TyExpression {
                                                                    expression: EnumInstantiation {
                                                                        enum_decl: TyEnumDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 857,
                                                                                    end: 866,
                                                                                    as_str(): "DumbError",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 873,
                                                                                            end: 878,
                                                                                            as_str(): "Error",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    type_id: TypeId(
                                                                                        7383,
                                                                                    ),
                                                                                    initial_type_id: TypeId(
                                                                                        7382,
                                                                                    ),
                                                                                    type_span: Span {
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 880,
                                                                                        end: 882,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                    tag: 0,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 873,
                                                                                        end: 882,
                                                                                        as_str(): "Error: ()",
                                                                                    },
                                                                                    attributes: {},
                                                                                },
                                                                            ],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 848,
                                                                                end: 885,
                                                                                as_str(): "pub enum DumbError {\n    Error: (),\n}",
                                                                            },
                                                                            visibility: Public,
                                                                        },
                                                                        variant_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 873,
                                                                                end: 878,
                                                                                as_str(): "Error",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        tag: 0,
                                                                        contents: None,
                                                                        enum_instantiation_span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1074,
                                                                            end: 1083,
                                                                            as_str(): "DumbError",
                                                                        },
                                                                        variant_instantiation_span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1085,
                                                                            end: 1090,
                                                                            as_str(): "Error",
                                                                        },
                                                                        type_binding: TypeBinding {
                                                                            inner: (),
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1085,
                                                                                end: 1090,
                                                                                as_str(): "Error",
                                                                            },
                                                                        },
                                                                    },
                                                                    return_type: TypeId(
                                                                        7406,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1085,
                                                                        end: 1090,
                                                                        as_str(): "Error",
                                                                    },
                                                                },
                                                            ),
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1062,
                                                                end: 1068,
                                                                as_str(): "Rezult",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1070,
                                                                end: 1073,
                                                                as_str(): "Err",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1062,
                                                                    end: 1073,
                                                                    as_str(): "Rezult::Err",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            7423,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1070,
                                                            end: 1073,
                                                            as_str(): "Err",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 1062,
                                                    end: 1091,
                                                    as_str(): "Rezult::Err(DumbError::Error)",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7423,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 1052,
                                    end: 1097,
                                    as_str(): "{\n        Rezult::Err(DumbError::Error)\n    }",
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
                                                            expression: EnumInstantiation {
                                                                enum_decl: TyEnumDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 807,
                                                                            end: 813,
                                                                            as_str(): "Rezult",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_parameters: [
                                                                        T: TypeId(7425),
                                                                        E: TypeId(7426),
                                                                    ],
                                                                    attributes: {},
                                                                    variants: [
                                                                        TyEnumVariant {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 826,
                                                                                    end: 828,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_id: TypeId(
                                                                                7425,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                7380,
                                                                            ),
                                                                            type_span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 830,
                                                                                end: 831,
                                                                                as_str(): "T",
                                                                            },
                                                                            tag: 0,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 826,
                                                                                end: 831,
                                                                                as_str(): "Ok: T",
                                                                            },
                                                                            attributes: {},
                                                                        },
                                                                        TyEnumVariant {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 837,
                                                                                    end: 840,
                                                                                    as_str(): "Err",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_id: TypeId(
                                                                                7426,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                7381,
                                                                            ),
                                                                            type_span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 842,
                                                                                end: 843,
                                                                                as_str(): "E",
                                                                            },
                                                                            tag: 1,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 837,
                                                                                end: 843,
                                                                                as_str(): "Err: E",
                                                                            },
                                                                            attributes: {},
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 798,
                                                                        end: 846,
                                                                        as_str(): "pub enum Rezult<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                                                    },
                                                                    visibility: Public,
                                                                },
                                                                variant_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 826,
                                                                        end: 828,
                                                                        as_str(): "Ok",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                tag: 0,
                                                                contents: Some(
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U8(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            50,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1124,
                                                                            end: 1127,
                                                                            as_str(): "1u8",
                                                                        },
                                                                    },
                                                                ),
                                                                enum_instantiation_span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1113,
                                                                    end: 1119,
                                                                    as_str(): "Rezult",
                                                                },
                                                                variant_instantiation_span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1121,
                                                                    end: 1123,
                                                                    as_str(): "Ok",
                                                                },
                                                                type_binding: TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1113,
                                                                        end: 1123,
                                                                        as_str(): "Rezult::Ok",
                                                                    },
                                                                },
                                                            },
                                                            return_type: TypeId(
                                                                7431,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1121,
                                                                end: 1123,
                                                                as_str(): "Ok",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 1113,
                                                        end: 1128,
                                                        as_str(): "Rezult::Ok(1u8)",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7431,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1103,
                                        end: 1134,
                                        as_str(): "{\n        Rezult::Ok(1u8)\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7423,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 1037,
                            end: 1134,
                            as_str(): "if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 1037,
                    end: 1134,
                    as_str(): "if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 980,
                    end: 981,
                    as_str(): "r",
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
                7407,
            ),
            initial_type_id: TypeId(
                7405,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 983,
                end: 1004,
                as_str(): "Rezult<u8, DumbError>",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 967,
        end: 1136,
        as_str(): "pub fn func5(r: Rezult<u8, DumbError>) -> Rezult<u8, DumbError> {\n    if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        7411,
    ),
    initial_return_type: TypeId(
        7410,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 1009,
        end: 1030,
        as_str(): "Rezult<u8, DumbError>",
    },
    visibility: Public,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 1145,
            end: 1150,
            as_str(): "func6",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 1216,
                                                end: 1223,
                                                as_str(): "trivial",
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 928,
                                                    end: 932,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1151,
                                                            end: 1152,
                                                            as_str(): "r",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 1214,
                                                        end: 1215,
                                                        as_str(): "r",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7434,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 1214,
                                                    end: 1215,
                                                    as_str(): "r",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        586,
                                        Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 917,
                                            end: 963,
                                            as_str(): "fn trivial(self) -> bool {\n        false\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 1216,
                                                end: 1223,
                                                as_str(): "trivial",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 1214,
                                    end: 1225,
                                    as_str(): "r.trivial()",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 807,
                                                                        end: 813,
                                                                        as_str(): "Rezult",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_parameters: [
                                                                    T: TypeId(7444),
                                                                    E: TypeId(7445),
                                                                ],
                                                                attributes: {},
                                                                variants: [
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 826,
                                                                                end: 828,
                                                                                as_str(): "Ok",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            7444,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            7380,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 830,
                                                                            end: 831,
                                                                            as_str(): "T",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 826,
                                                                            end: 831,
                                                                            as_str(): "Ok: T",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 837,
                                                                                end: 840,
                                                                                as_str(): "Err",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            7445,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            7381,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 842,
                                                                            end: 843,
                                                                            as_str(): "E",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 837,
                                                                            end: 843,
                                                                            as_str(): "Err: E",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 798,
                                                                    end: 846,
                                                                    as_str(): "pub enum Rezult<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                                                },
                                                                visibility: Public,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 837,
                                                                    end: 840,
                                                                    as_str(): "Err",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 1,
                                                            contents: Some(
                                                                TyExpression {
                                                                    expression: EnumInstantiation {
                                                                        enum_decl: TyEnumDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 857,
                                                                                    end: 866,
                                                                                    as_str(): "DumbError",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 873,
                                                                                            end: 878,
                                                                                            as_str(): "Error",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    type_id: TypeId(
                                                                                        7383,
                                                                                    ),
                                                                                    initial_type_id: TypeId(
                                                                                        7382,
                                                                                    ),
                                                                                    type_span: Span {
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 880,
                                                                                        end: 882,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                    tag: 0,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 873,
                                                                                        end: 882,
                                                                                        as_str(): "Error: ()",
                                                                                    },
                                                                                    attributes: {},
                                                                                },
                                                                            ],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 848,
                                                                                end: 885,
                                                                                as_str(): "pub enum DumbError {\n    Error: (),\n}",
                                                                            },
                                                                            visibility: Public,
                                                                        },
                                                                        variant_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 873,
                                                                                end: 878,
                                                                                as_str(): "Error",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        tag: 0,
                                                                        contents: None,
                                                                        enum_instantiation_span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1248,
                                                                            end: 1257,
                                                                            as_str(): "DumbError",
                                                                        },
                                                                        variant_instantiation_span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1259,
                                                                            end: 1264,
                                                                            as_str(): "Error",
                                                                        },
                                                                        type_binding: TypeBinding {
                                                                            inner: (),
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1259,
                                                                                end: 1264,
                                                                                as_str(): "Error",
                                                                            },
                                                                        },
                                                                    },
                                                                    return_type: TypeId(
                                                                        7406,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1259,
                                                                        end: 1264,
                                                                        as_str(): "Error",
                                                                    },
                                                                },
                                                            ),
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1236,
                                                                end: 1242,
                                                                as_str(): "Rezult",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1244,
                                                                end: 1247,
                                                                as_str(): "Err",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1236,
                                                                    end: 1247,
                                                                    as_str(): "Rezult::Err",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            7450,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1244,
                                                            end: 1247,
                                                            as_str(): "Err",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 1236,
                                                    end: 1265,
                                                    as_str(): "Rezult::Err(DumbError::Error)",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7450,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 1226,
                                    end: 1271,
                                    as_str(): "{\n        Rezult::Err(DumbError::Error)\n    }",
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
                                                            expression: EnumInstantiation {
                                                                enum_decl: TyEnumDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 807,
                                                                            end: 813,
                                                                            as_str(): "Rezult",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_parameters: [
                                                                        T: TypeId(7452),
                                                                        E: TypeId(7453),
                                                                    ],
                                                                    attributes: {},
                                                                    variants: [
                                                                        TyEnumVariant {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 826,
                                                                                    end: 828,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_id: TypeId(
                                                                                7452,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                7380,
                                                                            ),
                                                                            type_span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 830,
                                                                                end: 831,
                                                                                as_str(): "T",
                                                                            },
                                                                            tag: 0,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 826,
                                                                                end: 831,
                                                                                as_str(): "Ok: T",
                                                                            },
                                                                            attributes: {},
                                                                        },
                                                                        TyEnumVariant {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 837,
                                                                                    end: 840,
                                                                                    as_str(): "Err",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_id: TypeId(
                                                                                7453,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                7381,
                                                                            ),
                                                                            type_span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 842,
                                                                                end: 843,
                                                                                as_str(): "E",
                                                                            },
                                                                            tag: 1,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 837,
                                                                                end: 843,
                                                                                as_str(): "Err: E",
                                                                            },
                                                                            attributes: {},
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 798,
                                                                        end: 846,
                                                                        as_str(): "pub enum Rezult<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                                                    },
                                                                    visibility: Public,
                                                                },
                                                                variant_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 826,
                                                                        end: 828,
                                                                        as_str(): "Ok",
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1298,
                                                                            end: 1302,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                                enum_instantiation_span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1287,
                                                                    end: 1293,
                                                                    as_str(): "Rezult",
                                                                },
                                                                variant_instantiation_span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1295,
                                                                    end: 1297,
                                                                    as_str(): "Ok",
                                                                },
                                                                type_binding: TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1287,
                                                                        end: 1297,
                                                                        as_str(): "Rezult::Ok",
                                                                    },
                                                                },
                                                            },
                                                            return_type: TypeId(
                                                                7458,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1295,
                                                                end: 1297,
                                                                as_str(): "Ok",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 1287,
                                                        end: 1303,
                                                        as_str(): "Rezult::Ok(true)",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7458,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1277,
                                        end: 1309,
                                        as_str(): "{\n        Rezult::Ok(true)\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7450,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 1211,
                            end: 1309,
                            as_str(): "if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 1211,
                    end: 1309,
                    as_str(): "if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 1151,
                    end: 1152,
                    as_str(): "r",
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
                7434,
            ),
            initial_type_id: TypeId(
                7433,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fc38b640,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                ),
                start: 1154,
                end: 1177,
                as_str(): "Rezult<bool, DumbError>",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 1138,
        end: 1311,
        as_str(): "pub fn func6(r: Rezult<bool, DumbError>) -> Rezult<bool, DumbError> {\n   if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        7438,
    ),
    initial_return_type: TypeId(
        7437,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 1182,
        end: 1205,
        as_str(): "Rezult<bool, DumbError>",
    },
    visibility: Public,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 1341,
            end: 1345,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
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
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 1360,
                            end: 1364,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc38b640,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                    ),
                    start: 1360,
                    end: 1364,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 1338,
        end: 1366,
        as_str(): "fn main() -> bool {\n  true\n}",
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
        src (ptr): 0x00007fe0fc38b640,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
        ),
        start: 1351,
        end: 1355,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

