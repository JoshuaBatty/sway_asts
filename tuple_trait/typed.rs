
ImplTrait(
    DeclId(
        13329,
        Span {
            src (ptr): 0x00007f8a1d5dbf10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
            ),
            start: 28,
            end: 172,
            as_str(): "impl Eq for (u64, u64, u64) {\n    fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1 && self.2 == other.2\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        13342,
        Span {
            src (ptr): 0x00007f8a1d5dbf10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
            ),
            start: 174,
            end: 292,
            as_str(): "impl Eq for (u64, u64) {\n    fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a1d5dbf10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
            ),
            start: 297,
            end: 301,
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
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 322,
                                    end: 323,
                                    as_str(): "t",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [
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
                                                src (ptr): 0x00007f8a1d5dbf10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                ),
                                                start: 327,
                                                end: 329,
                                                as_str(): "42",
                                            },
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
                                                src (ptr): 0x00007f8a1d5dbf10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                ),
                                                start: 331,
                                                end: 333,
                                                as_str(): "43",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    32025,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 326,
                                    end: 334,
                                    as_str(): "(42, 43)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                32025,
                            ),
                            type_ascription: TypeId(
                                32019,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a1d5dbf10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                    ),
                    start: 318,
                    end: 335,
                    as_str(): "let t = (42, 43);",
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
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 340,
                                        end: 346,
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
                                            src (ptr): 0x00007f8a217c74d0,
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
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 349,
                                                            end: 351,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 349,
                                                            end: 351,
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
                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                        ),
                                                        start: 349,
                                                        end: 351,
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
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 209,
                                                            end: 213,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                    ),
                                                                    start: 322,
                                                                    end: 323,
                                                                    as_str(): "t",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 347,
                                                                end: 348,
                                                                as_str(): "t",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            32029,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 348,
                                                            as_str(): "t",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 215,
                                                            end: 220,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                    ),
                                                                    start: 322,
                                                                    end: 323,
                                                                    as_str(): "t",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 352,
                                                                end: 353,
                                                                as_str(): "t",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            32031,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 352,
                                                            end: 353,
                                                            as_str(): "t",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13344,
                                                Span {
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 203,
                                                    end: 290,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                        ),
                                                        start: 349,
                                                        end: 351,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 353,
                                            as_str(): "t == t",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13345,
                                Span {
                                    src (ptr): 0x00007f8a217c74d0,
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
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 340,
                                        end: 346,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32033,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a1d5dbf10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                            ),
                            start: 340,
                            end: 354,
                            as_str(): "assert(t == t)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a1d5dbf10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                    ),
                    start: 340,
                    end: 354,
                    as_str(): "assert(t == t)",
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
                            src (ptr): 0x00007f8a1d5dbf10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                            ),
                            start: 361,
                            end: 365,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a1d5dbf10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                    ),
                    start: 361,
                    end: 365,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a1d5dbf10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
        ),
        start: 294,
        end: 367,
        as_str(): "fn main() -> bool {\n    let t = (42, 43);\n    assert(t == t);\n\n    true\n}",
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
        src (ptr): 0x00007f8a1d5dbf10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
        ),
        start: 307,
        end: 311,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

