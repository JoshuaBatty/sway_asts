TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12e8db920,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                    src (ptr): 0x00007fb12e8db920,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                src (ptr): 0x00007fb12e8db920,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                ),
                start: 35,
                end: 36,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb12e8db920,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                    src (ptr): 0x00007fb12e8db920,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                src (ptr): 0x00007fb12e8db920,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                ),
                start: 45,
                end: 46,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb12e8db920,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                ),
                start: 40,
                end: 46,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb12e8db920,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
            src (ptr): 0x00007fb12e8db920,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
            ),
            start: 73,
            end: 77,
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
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 103,
                                    as_str(): "result_a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb12e8db920,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                ),
                                                start: 14,
                                                end: 20,
                                                as_str(): "Result",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(21),
                                            E: TypeId(71),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb12e8db920,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 35,
                                                    end: 36,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                        src (ptr): 0x00007fb12e8db920,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                        ),
                                                        start: 40,
                                                        end: 43,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    7256,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 46,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 46,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                src (ptr): 0x00007fb12e8db920,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                ),
                                                start: 130,
                                                end: 134,
                                                as_str(): "5u64",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 106,
                                        end: 112,
                                        as_str(): "Result",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 114,
                                        end: 116,
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 122,
                                                    as_str(): "u64",
                                                },
                                            },
                                            TypeArgument {
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    7250,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 124,
                                                    end: 128,
                                                    as_str(): "bool",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                            ),
                                            start: 106,
                                            end: 129,
                                            as_str(): "Result::Ok::<u64, bool>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7261,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 116,
                                    as_str(): "Ok",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7261,
                            ),
                            type_ascription: TypeId(
                                7258,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12e8db920,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                    ),
                    start: 91,
                    end: 136,
                    as_str(): "let result_a = Result::Ok::<u64, bool>(5u64);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 143,
                                    end: 151,
                                    as_str(): "result_b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb12e8db920,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                ),
                                                start: 14,
                                                end: 20,
                                                as_str(): "Result",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(21),
                                            E: TypeId(71),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb12e8db920,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 35,
                                                    end: 36,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                                        src (ptr): 0x00007fb12e8db920,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                        ),
                                                        start: 40,
                                                        end: 43,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    7256,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 46,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 46,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
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
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 43,
                                            as_str(): "Err",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 1,
                                    contents: Some(
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
                                                src (ptr): 0x00007fb12e8db920,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                ),
                                                start: 179,
                                                end: 184,
                                                as_str(): "false",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 154,
                                        end: 160,
                                        as_str(): "Result",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 162,
                                        end: 165,
                                        as_str(): "Err",
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 168,
                                                    end: 171,
                                                    as_str(): "u64",
                                                },
                                            },
                                            TypeArgument {
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    7250,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 173,
                                                    end: 177,
                                                    as_str(): "bool",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                            ),
                                            start: 154,
                                            end: 178,
                                            as_str(): "Result::Err::<u64, bool>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7265,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 162,
                                    end: 165,
                                    as_str(): "Err",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7265,
                            ),
                            type_ascription: TypeId(
                                7262,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12e8db920,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                    ),
                    start: 139,
                    end: 186,
                    as_str(): "let result_b = Result::Err::<u64, bool>(false);",
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 214,
                                                    end: 222,
                                                    as_str(): "result_a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 214,
                                                    end: 222,
                                                    as_str(): "result_a",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb12e8db920,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                ),
                                                start: 214,
                                                end: 222,
                                                as_str(): "result_a",
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
                                                    src (ptr): 0x00007fb132fce890,
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
                                                                    src (ptr): 0x00007fb12e8db920,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 103,
                                                                    as_str(): "result_a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 214,
                                                                end: 222,
                                                                as_str(): "result_a",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            7261,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 214,
                                                            end: 222,
                                                            as_str(): "result_a",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 214,
                                                    end: 222,
                                                    as_str(): "result_a",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb132fce890,
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 214,
                                                    end: 222,
                                                    as_str(): "result_a",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        549,
                                        Span {
                                            src (ptr): 0x00007fb132fce890,
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
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 214,
                                    end: 222,
                                    as_str(): "result_a",
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
                                                                    src (ptr): 0x00007fb12e8db920,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                    ),
                                                                    start: 209,
                                                                    end: 210,
                                                                    as_str(): "a",
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
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 95,
                                                                                    end: 103,
                                                                                    as_str(): "result_a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 214,
                                                                                end: 222,
                                                                                as_str(): "result_a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            7261,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 214,
                                                                            end: 222,
                                                                            as_str(): "result_a",
                                                                        },
                                                                    },
                                                                    variant: TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 40,
                                                                                end: 43,
                                                                                as_str(): "Err",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            7268,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            7256,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 45,
                                                                            end: 46,
                                                                            as_str(): "E",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 40,
                                                                            end: 46,
                                                                            as_str(): "Err: E",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    7268,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12e8db920,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                    ),
                                                                    start: 197,
                                                                    end: 211,
                                                                    as_str(): "Result::Err(a)",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7268,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7268,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 209,
                                                    end: 210,
                                                    as_str(): "a",
                                                },
                                            },
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                6,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 229,
                                                            end: 230,
                                                            as_str(): "6",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 229,
                                                    end: 230,
                                                    as_str(): "6",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 223,
                                    end: 234,
                                    as_str(): "{\n    6\n  }",
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
                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 265,
                                                                                        end: 273,
                                                                                        as_str(): "result_b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 265,
                                                                                        end: 273,
                                                                                        as_str(): "result_b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 265,
                                                                                    end: 273,
                                                                                    as_str(): "result_b",
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
                                                                                        src (ptr): 0x00007fb132fce890,
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
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 143,
                                                                                                        end: 151,
                                                                                                        as_str(): "result_b",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 265,
                                                                                                    end: 273,
                                                                                                    as_str(): "result_b",
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                7265,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 265,
                                                                                                end: 273,
                                                                                                as_str(): "result_b",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 265,
                                                                                        end: 273,
                                                                                        as_str(): "result_b",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb132fce890,
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
                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 265,
                                                                                        end: 273,
                                                                                        as_str(): "result_b",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            548,
                                                                            Span {
                                                                                src (ptr): 0x00007fb132fce890,
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
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 265,
                                                                        end: 273,
                                                                        as_str(): "result_b",
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
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 258,
                                                                                                        end: 261,
                                                                                                        as_str(): "num",
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
                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 143,
                                                                                                                        end: 151,
                                                                                                                        as_str(): "result_b",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 265,
                                                                                                                    end: 273,
                                                                                                                    as_str(): "result_b",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7265,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 265,
                                                                                                                end: 273,
                                                                                                                as_str(): "result_b",
                                                                                                            },
                                                                                                        },
                                                                                                        variant: TyEnumVariant {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 31,
                                                                                                                    end: 33,
                                                                                                                    as_str(): "Ok",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_id: TypeId(
                                                                                                                7277,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                7255,
                                                                                                            ),
                                                                                                            type_span: Span {
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 35,
                                                                                                                end: 36,
                                                                                                                as_str(): "T",
                                                                                                            },
                                                                                                            tag: 0,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 31,
                                                                                                                end: 36,
                                                                                                                as_str(): "Ok: T",
                                                                                                            },
                                                                                                            attributes: {},
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        7277,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 247,
                                                                                                        end: 262,
                                                                                                        as_str(): "Result::Ok(num)",
                                                                                                    },
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                                return_type: TypeId(
                                                                                                    7277,
                                                                                                ),
                                                                                                type_ascription: TypeId(
                                                                                                    7277,
                                                                                                ),
                                                                                                type_ascription_span: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 258,
                                                                                        end: 261,
                                                                                        as_str(): "num",
                                                                                    },
                                                                                },
                                                                                TyAstNode {
                                                                                    content: ImplicitReturnExpression(
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
                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 280,
                                                                                                end: 282,
                                                                                                as_str(): "10",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 280,
                                                                                        end: 282,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 274,
                                                                        end: 286,
                                                                        as_str(): "{\n    10\n  }",
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
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 317,
                                                                                                                            end: 325,
                                                                                                                            as_str(): "result_a",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 317,
                                                                                                                            end: 325,
                                                                                                                            as_str(): "result_a",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ],
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "eq",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 317,
                                                                                                                        end: 325,
                                                                                                                        as_str(): "result_a",
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
                                                                                                                            src (ptr): 0x00007fb132fce890,
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
                                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 95,
                                                                                                                                            end: 103,
                                                                                                                                            as_str(): "result_a",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 317,
                                                                                                                                        end: 325,
                                                                                                                                        as_str(): "result_a",
                                                                                                                                    },
                                                                                                                                    mutability: Immutable,
                                                                                                                                },
                                                                                                                                return_type: TypeId(
                                                                                                                                    7261,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 317,
                                                                                                                                    end: 325,
                                                                                                                                    as_str(): "result_a",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            21,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 317,
                                                                                                                            end: 325,
                                                                                                                            as_str(): "result_a",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                (
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb132fce890,
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
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 317,
                                                                                                                            end: 325,
                                                                                                                            as_str(): "result_a",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                            function_decl_id: DeclId(
                                                                                                                547,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fb132fce890,
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
                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 317,
                                                                                                            end: 325,
                                                                                                            as_str(): "result_a",
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
                                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 310,
                                                                                                                                            end: 313,
                                                                                                                                            as_str(): "num",
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
                                                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 95,
                                                                                                                                                            end: 103,
                                                                                                                                                            as_str(): "result_a",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 317,
                                                                                                                                                        end: 325,
                                                                                                                                                        as_str(): "result_a",
                                                                                                                                                    },
                                                                                                                                                    mutability: Immutable,
                                                                                                                                                },
                                                                                                                                                return_type: TypeId(
                                                                                                                                                    7261,
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 317,
                                                                                                                                                    end: 325,
                                                                                                                                                    as_str(): "result_a",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                            variant: TyEnumVariant {
                                                                                                                                                name: BaseIdent {
                                                                                                                                                    name_override_opt: None,
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 31,
                                                                                                                                                        end: 33,
                                                                                                                                                        as_str(): "Ok",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                type_id: TypeId(
                                                                                                                                                    7287,
                                                                                                                                                ),
                                                                                                                                                initial_type_id: TypeId(
                                                                                                                                                    7255,
                                                                                                                                                ),
                                                                                                                                                type_span: Span {
                                                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 35,
                                                                                                                                                    end: 36,
                                                                                                                                                    as_str(): "T",
                                                                                                                                                },
                                                                                                                                                tag: 0,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 31,
                                                                                                                                                    end: 36,
                                                                                                                                                    as_str(): "Ok: T",
                                                                                                                                                },
                                                                                                                                                attributes: {},
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        return_type: TypeId(
                                                                                                                                            7287,
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 299,
                                                                                                                                            end: 314,
                                                                                                                                            as_str(): "Result::Ok(num)",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    mutability: Immutable,
                                                                                                                                    return_type: TypeId(
                                                                                                                                        7287,
                                                                                                                                    ),
                                                                                                                                    type_ascription: TypeId(
                                                                                                                                        7287,
                                                                                                                                    ),
                                                                                                                                    type_ascription_span: None,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 310,
                                                                                                                            end: 313,
                                                                                                                            as_str(): "num",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    TyAstNode {
                                                                                                                        content: ImplicitReturnExpression(
                                                                                                                            TyExpression {
                                                                                                                                expression: VariableExpression {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 310,
                                                                                                                                            end: 313,
                                                                                                                                            as_str(): "num",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 332,
                                                                                                                                        end: 335,
                                                                                                                                        as_str(): "num",
                                                                                                                                    },
                                                                                                                                    mutability: Immutable,
                                                                                                                                },
                                                                                                                                return_type: TypeId(
                                                                                                                                    7287,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 332,
                                                                                                                                    end: 335,
                                                                                                                                    as_str(): "num",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 332,
                                                                                                                            end: 335,
                                                                                                                            as_str(): "num",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            7287,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 326,
                                                                                                            end: 339,
                                                                                                            as_str(): "{\n    num\n  }",
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
                                                                                                                                            42,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                    return_type: TypeId(
                                                                                                                                        21,
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 352,
                                                                                                                                        end: 354,
                                                                                                                                        as_str(): "42",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 352,
                                                                                                                                end: 354,
                                                                                                                                as_str(): "42",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            return_type: TypeId(
                                                                                                                21,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 345,
                                                                                                                end: 359,
                                                                                                                as_str(): "{ \n    42 \n  }",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7287,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 326,
                                                                                                    end: 339,
                                                                                                    as_str(): "{\n    num\n  }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 326,
                                                                                            end: 339,
                                                                                            as_str(): "{\n    num\n  }",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            7287,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 326,
                                                                            end: 339,
                                                                            as_str(): "{\n    num\n  }",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 274,
                                                                end: 286,
                                                                as_str(): "{\n    10\n  }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb12e8db920,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                        ),
                                                        start: 274,
                                                        end: 286,
                                                        as_str(): "{\n    10\n  }",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 274,
                                        end: 286,
                                        as_str(): "{\n    10\n  }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12e8db920,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                            ),
                            start: 223,
                            end: 234,
                            as_str(): "{\n    6\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12e8db920,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                    ),
                    start: 190,
                    end: 359,
                    as_str(): "if let Result::Err(a) = result_a {\n    6\n  } else if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12e8db920,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
        ),
        start: 70,
        end: 361,
        as_str(): "fn main() -> u64 {\n  let result_a = Result::Ok::<u64, bool>(5u64);\n  let result_b = Result::Err::<u64, bool>(false);\n\n  if let Result::Err(a) = result_a {\n    6\n  } else if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }\n}",
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
        src (ptr): 0x00007fb12e8db920,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
        ),
        start: 83,
        end: 86,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

