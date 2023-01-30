TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb10d4510c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
            ),
            start: 14,
            end: 20,
            as_str(): "Result",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7253),
        E: TypeId(7254),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb10d4510c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                    ),
                    start: 31,
                    end: 33,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7253,
            ),
            initial_type_id: TypeId(
                7255,
            ),
            type_span: Span {
                src (ptr): 0x00007fb10d4510c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                ),
                start: 35,
                end: 36,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb10d4510c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                ),
                start: 31,
                end: 36,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb10d4510c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                    ),
                    start: 40,
                    end: 43,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7254,
            ),
            initial_type_id: TypeId(
                7256,
            ),
            type_span: Span {
                src (ptr): 0x00007fb10d4510c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                ),
                start: 45,
                end: 46,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb10d4510c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                ),
                start: 40,
                end: 46,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb10d4510c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
        ),
        start: 9,
        end: 49,
        as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb10d4510c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
            ),
            start: 54,
            end: 58,
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
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                    ),
                                    start: 77,
                                    end: 78,
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
                                                src (ptr): 0x00007fb10d4510c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                ),
                                                start: 14,
                                                end: 20,
                                                as_str(): "Result",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(21),
                                            E: TypeId(21),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d4510c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                        ),
                                                        start: 31,
                                                        end: 33,
                                                        as_str(): "Ok",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7255,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 35,
                                                    end: 36,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 31,
                                                    end: 36,
                                                    as_str(): "Ok: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d4510c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                        ),
                                                        start: 40,
                                                        end: 43,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7256,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 46,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 46,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb10d4510c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 49,
                                            as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb10d4510c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                            ),
                                            start: 31,
                                            end: 33,
                                            as_str(): "Ok",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
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
                                                src (ptr): 0x00007fb10d4510c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                ),
                                                start: 122,
                                                end: 126,
                                                as_str(): "5u64",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                        ),
                                        start: 99,
                                        end: 105,
                                        as_str(): "Result",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                        ),
                                        start: 107,
                                        end: 109,
                                        as_str(): "Ok",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [
                                            TypeArgument {
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7249,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 112,
                                                    end: 115,
                                                    as_str(): "u64",
                                                },
                                            },
                                            TypeArgument {
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7250,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 117,
                                                    end: 120,
                                                    as_str(): "u64",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb10d4510c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                            ),
                                            start: 99,
                                            end: 121,
                                            as_str(): "Result::Ok::<u64, u64>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7262,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                    ),
                                    start: 107,
                                    end: 109,
                                    as_str(): "Ok",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7262,
                            ),
                            type_ascription: TypeId(
                                7259,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                    ),
                                    start: 80,
                                    end: 96,
                                    as_str(): "Result<u64, u64>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb10d4510c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                    ),
                    start: 73,
                    end: 128,
                    as_str(): "let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);",
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
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 181,
                                                    end: 182,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 181,
                                                    end: 182,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb10d4510c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                ),
                                                start: 181,
                                                end: 182,
                                                as_str(): "x",
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
                                                    src (ptr): 0x00007fb1150556b0,
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
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 77,
                                                                    end: 78,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 181,
                                                                end: 182,
                                                                as_str(): "x",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            7262,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 181,
                                                            end: 182,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 181,
                                                    end: 182,
                                                    as_str(): "x",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1150556b0,
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
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 181,
                                                    end: 182,
                                                    as_str(): "x",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        547,
                                        Span {
                                            src (ptr): 0x00007fb1150556b0,
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
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                    ),
                                    start: 181,
                                    end: 182,
                                    as_str(): "x",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Declaration(
                                                    VariableDeclaration(
                                                        TyVariableDeclaration {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 176,
                                                                    end: 177,
                                                                    as_str(): "y",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: UnsafeDowncast {
                                                                    exp: TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 77,
                                                                                    end: 78,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 181,
                                                                                end: 182,
                                                                                as_str(): "x",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            7262,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 181,
                                                                            end: 182,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    variant: TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 31,
                                                                                end: 33,
                                                                                as_str(): "Ok",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            7264,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            7255,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 35,
                                                                            end: 36,
                                                                            as_str(): "T",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 31,
                                                                            end: 36,
                                                                            as_str(): "Ok: T",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    7264,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 165,
                                                                    end: 178,
                                                                    as_str(): "Result::Ok(y)",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7264,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7264,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 176,
                                                    end: 177,
                                                    as_str(): "y",
                                                },
                                            },
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
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 187,
                                                                            end: 188,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 187,
                                                                            end: 188,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "add",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 187,
                                                                        end: 188,
                                                                        as_str(): "+",
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
                                                                            src (ptr): 0x00007fb1150556b0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 124,
                                                                            end: 128,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 176,
                                                                                    end: 177,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 185,
                                                                                end: 186,
                                                                                as_str(): "y",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            7264,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 185,
                                                                            end: 186,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1150556b0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 130,
                                                                            end: 135,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
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
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 189,
                                                                            end: 191,
                                                                            as_str(): "10",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                546,
                                                                Span {
                                                                    src (ptr): 0x00007fb1150556b0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 185,
                                                                    as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 187,
                                                                        end: 188,
                                                                        as_str(): "+",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 185,
                                                            end: 191,
                                                            as_str(): "y + 10",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 185,
                                                    end: 191,
                                                    as_str(): "y + 10",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                    ),
                                    start: 183,
                                    end: 193,
                                    as_str(): "{ y + 10 }",
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
                                                            expression: Literal(
                                                                U64(
                                                                    1,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 201,
                                                                end: 202,
                                                                as_str(): "1",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d4510c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                        ),
                                                        start: 201,
                                                        end: 202,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                        ),
                                        start: 199,
                                        end: 204,
                                        as_str(): "{ 1 }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb10d4510c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                            ),
                            start: 183,
                            end: 193,
                            as_str(): "{ y + 10 }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb10d4510c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                    ),
                    start: 158,
                    end: 204,
                    as_str(): "if let Result::Ok(y) = x { y + 10 } else { 1 }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb10d4510c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
        ),
        start: 51,
        end: 206,
        as_str(): "fn main() -> u64 {\n   let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    // should return 15\n    if let Result::Ok(y) = x { y + 10 } else { 1 }\n}",
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
        src (ptr): 0x00007fb10d4510c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
        ),
        start: 64,
        end: 67,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

