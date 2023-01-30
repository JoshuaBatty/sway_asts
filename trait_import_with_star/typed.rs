

TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe033497b40,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
            ),
            start: 45,
            end: 49,
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
                                    src (ptr): 0x00007fe033497b40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                    ),
                                    start: 66,
                                    end: 77,
                                    as_str(): "shiftAnswer",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    7311,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe033497b40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                    ),
                                    start: 85,
                                    end: 86,
                                    as_str(): "0",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe033497b40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                    ),
                                    start: 79,
                                    end: 82,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe033497b40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                    ),
                    start: 58,
                    end: 87,
                    as_str(): "let mut shiftAnswer: u64 = 0;",
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
                                        src (ptr): 0x00007fe033497b40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                        ),
                                        start: 105,
                                        end: 108,
                                        as_str(): "rsh",
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
                                            src (ptr): 0x00007fe0332e1c50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                            ),
                                            start: 299,
                                            end: 303,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe033497b40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                    ),
                                                    start: 66,
                                                    end: 77,
                                                    as_str(): "shiftAnswer",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe033497b40,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                ),
                                                start: 93,
                                                end: 104,
                                                as_str(): "shiftAnswer",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                            ),
                                            start: 93,
                                            end: 104,
                                            as_str(): "shiftAnswer",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0332e1c50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                            ),
                                            start: 305,
                                            end: 310,
                                            as_str(): "other",
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
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                            ),
                                            start: 109,
                                            end: 110,
                                            as_str(): "5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                552,
                                Span {
                                    src (ptr): 0x00007fe0332e1c50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/shiftable.sw",
                                    ),
                                    start: 292,
                                    end: 427,
                                    as_str(): "fn rsh(self, other: u64) -> Self {\n        asm(r1: self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe033497b40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                        ),
                                        start: 105,
                                        end: 108,
                                        as_str(): "rsh",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe033497b40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                            ),
                            start: 93,
                            end: 111,
                            as_str(): "shiftAnswer.rsh(5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe033497b40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                    ),
                    start: 93,
                    end: 111,
                    as_str(): "shiftAnswer.rsh(5)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe033497b40,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
        ),
        start: 42,
        end: 114,
        as_str(): "fn main() {\n    let mut shiftAnswer: u64 = 0;\n\n    shiftAnswer.rsh(5);\n}",
    },
    attributes: {},
    return_type: TypeId(
        7310,
    ),
    initial_return_type: TypeId(
        7309,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe033497b40,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
        ),
        start: 42,
        end: 51,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

