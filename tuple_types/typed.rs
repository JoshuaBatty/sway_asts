TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 12,
            end: 24,
            as_str(): "gimme_a_unit",
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
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 37,
                                    end: 38,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [],
                                },
                                return_type: TypeId(
                                    7257,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 45,
                                    end: 47,
                                    as_str(): "()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7257,
                            ),
                            type_ascription: TypeId(
                                7255,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 40,
                                    end: 42,
                                    as_str(): "()",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 33,
                    end: 48,
                    as_str(): "let x: () = ();",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 37,
                                    end: 38,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007f8a28725630,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                ),
                                start: 53,
                                end: 54,
                                as_str(): "x",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7258,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 53,
                            end: 54,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 53,
                    end: 54,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 9,
        end: 56,
        as_str(): "fn gimme_a_unit() {\n    let x: () = ();\n    x\n}",
    },
    attributes: {},
    return_type: TypeId(
        7253,
    ),
    initial_return_type: TypeId(
        7252,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 9,
        end: 26,
        as_str(): "fn gimme_a_unit()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 61,
            end: 78,
            as_str(): "also_gimme_a_unit",
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
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 97,
                                    end: 98,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [],
                                },
                                return_type: TypeId(
                                    7265,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 107,
                                    as_str(): "()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7265,
                            ),
                            type_ascription: TypeId(
                                7263,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 100,
                                    end: 102,
                                    as_str(): "()",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 93,
                    end: 108,
                    as_str(): "let x: () = ();",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 97,
                                    end: 98,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007f8a28725630,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                ),
                                start: 113,
                                end: 114,
                                as_str(): "x",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7266,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 113,
                            end: 114,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 113,
                    end: 114,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 58,
        end: 116,
        as_str(): "fn also_gimme_a_unit() -> () {\n    let x: () = ();\n    x\n}",
    },
    attributes: {},
    return_type: TypeId(
        7261,
    ),
    initial_return_type: TypeId(
        7260,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 84,
        end: 86,
        as_str(): "()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 121,
            end: 141,
            as_str(): "gimme_a_single_value",
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
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 164,
                                    end: 165,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [
                                        TyExpression {
                                            expression: Literal(
                                                U32(
                                                    123,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                33,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a28725630,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                ),
                                                start: 177,
                                                end: 183,
                                                as_str(): "123u32",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7273,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 176,
                                    end: 185,
                                    as_str(): "(123u32,)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7273,
                            ),
                            type_ascription: TypeId(
                                7271,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 167,
                                    end: 173,
                                    as_str(): "(u32,)",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 160,
                    end: 186,
                    as_str(): "let x: (u32,) = (123u32,);",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 164,
                                    end: 165,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007f8a28725630,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                ),
                                start: 191,
                                end: 192,
                                as_str(): "x",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7274,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 191,
                            end: 192,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 191,
                    end: 192,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 118,
        end: 194,
        as_str(): "fn gimme_a_single_value() -> (u32,) {\n    let x: (u32,) = (123u32,);\n    x\n}",
    },
    attributes: {},
    return_type: TypeId(
        7269,
    ),
    initial_return_type: TypeId(
        7268,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 147,
        end: 153,
        as_str(): "(u32,)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 199,
            end: 211,
            as_str(): "gimme_a_pair",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Tuple {
                            fields: [
                                TyExpression {
                                    expression: Literal(
                                        U32(
                                            1,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        33,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 235,
                                        end: 239,
                                        as_str(): "1u32",
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
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 241,
                                        end: 245,
                                        as_str(): "2u64",
                                    },
                                },
                            ],
                        },
                        return_type: TypeId(
                            7279,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 234,
                            end: 246,
                            as_str(): "(1u32, 2u64)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 234,
                    end: 246,
                    as_str(): "(1u32, 2u64)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 196,
        end: 248,
        as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
    },
    attributes: {},
    return_type: TypeId(
        7277,
    ),
    initial_return_type: TypeId(
        7276,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 217,
        end: 227,
        as_str(): "(u32, u64)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 253,
            end: 257,
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
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 273,
                                        end: 285,
                                        as_str(): "gimme_a_unit",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                549,
                                Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 56,
                                    as_str(): "fn gimme_a_unit() {\n    let x: () = ();\n    x\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 273,
                                        end: 285,
                                        as_str(): "gimme_a_unit",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7282,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 273,
                            end: 287,
                            as_str(): "gimme_a_unit()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 273,
                    end: 287,
                    as_str(): "gimme_a_unit()",
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
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 293,
                                        end: 310,
                                        as_str(): "also_gimme_a_unit",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                551,
                                Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 58,
                                    end: 116,
                                    as_str(): "fn also_gimme_a_unit() -> () {\n    let x: () = ();\n    x\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 293,
                                        end: 310,
                                        as_str(): "also_gimme_a_unit",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7284,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 293,
                            end: 312,
                            as_str(): "also_gimme_a_unit()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 293,
                    end: 312,
                    as_str(): "also_gimme_a_unit()",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 322,
                                    end: 323,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007f8a28725630,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                ),
                                                start: 326,
                                                end: 346,
                                                as_str(): "gimme_a_single_value",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        553,
                                        Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 118,
                                            end: 194,
                                            as_str(): "fn gimme_a_single_value() -> (u32,) {\n    let x: (u32,) = (123u32,);\n    x\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a28725630,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                ),
                                                start: 326,
                                                end: 346,
                                                as_str(): "gimme_a_single_value",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7286,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 326,
                                    end: 348,
                                    as_str(): "gimme_a_single_value()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7286,
                            ),
                            type_ascription: TypeId(
                                7285,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 318,
                    end: 349,
                    as_str(): "let x = gimme_a_single_value();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 358,
                                    end: 359,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007f8a28725630,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                ),
                                                start: 362,
                                                end: 374,
                                                as_str(): "gimme_a_pair",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        555,
                                        Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 196,
                                            end: 248,
                                            as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007f8a28725630,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                ),
                                                start: 362,
                                                end: 374,
                                                as_str(): "gimme_a_pair",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7288,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 362,
                                    end: 376,
                                    as_str(): "gimme_a_pair()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7288,
                            ),
                            type_ascription: TypeId(
                                7287,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 354,
                    end: 377,
                    as_str(): "let b = gimme_a_pair();",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U32(
                                123,
                            ),
                        ),
                        return_type: TypeId(
                            7289,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 382,
                            end: 385,
                            as_str(): "123",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a28725630,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                    ),
                    start: 382,
                    end: 385,
                    as_str(): "123",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 250,
        end: 387,
        as_str(): "fn main() -> u32 {\n    gimme_a_unit();\n    also_gimme_a_unit();\n    let x = gimme_a_single_value();\n    let b = gimme_a_pair();\n    123\n}",
    },
    attributes: {},
    return_type: TypeId(
        33,
    ),
    initial_return_type: TypeId(
        33,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007f8a28725630,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
        ),
        start: 263,
        end: 266,
        as_str(): "u32",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

